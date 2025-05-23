/*
 * Copyright © 2019 Google, Inc.
 * SPDX-License-Identifier: MIT
 */

#include "compiler/nir/nir_builder.h"
#include "ir3_nir.h"

/**
 * This pass lowers load_barycentric_at_offset to dsx.3d/dsy.3d and alu
 * instructions.
 */

static nir_def *
ir3_nir_lower_load_barycentric_at_offset_instr(nir_builder *b, nir_instr *instr,
                                               void *data)
{
   nir_intrinsic_instr *intr = nir_instr_as_intrinsic(instr);
   enum glsl_interp_mode interp_mode = nir_intrinsic_interp_mode(intr);

#define chan(var, c) nir_channel(b, var, c)

   nir_def *off = intr->src[0].ssa;
   /* note: at_offset is defined to be relative to the center of the pixel */
   nir_def *ij = nir_load_barycentric_pixel(b, 32, .interp_mode = interp_mode);

   /* Need helper invocations for our ddx/ddys to work. */
   if (b->shader->info.stage == MESA_SHADER_FRAGMENT)
      b->shader->info.fs.needs_coarse_quad_helper_invocations = true;

   if (interp_mode != INTERP_MODE_SMOOTH) {
      /* Offset our pixel center ij by the offset argument (units of pixels)
       * times the derivatives of ij in screen space.
       */
      nir_def *new_ij = ij;
      new_ij = nir_ffma(b, chan(off, 0), nir_ddx(b, ij), new_ij);
      new_ij = nir_ffma(b, chan(off, 1), nir_ddy(b, ij), new_ij);

      return new_ij;
   } else {
      nir_def *center_w = nir_frcp(b, nir_load_persp_center_rhw_ir3(b, 32));

      /* scaled ij -- ij comes in multiplied by by 1/center_w so multiply that
       * back out, plus add center_w as the 3rd component for taking the
       * derivatives.
       *
       * We actually suspect that we should be using rhw here instead of center_w,
       * but no tests seem to distinguish between the two.
       */
      nir_def *sij =
         nir_vec3(b, nir_fmul(b, chan(ij, 0), center_w), nir_fmul(b, chan(ij, 1), center_w), center_w);

      /* Get the offset value from pixel center for ij, and also for w. */
      nir_def *pos = sij;
      pos = nir_ffma(b, chan(off, 0), nir_ddx(b, sij), pos);
      pos = nir_ffma(b, chan(off, 1), nir_ddy(b, sij), pos);

      /* convert back into screen space, dividing by the offset 1/w */
      return nir_fmul(b, nir_trim_vector(b, pos, 2),
                      nir_frcp(b, chan(pos, 2)));
   }
}

static bool
ir3_nir_lower_load_barycentric_at_offset_filter(const nir_instr *instr,
                                                const void *data)
{
   return (instr->type == nir_instr_type_intrinsic &&
           nir_instr_as_intrinsic(instr)->intrinsic ==
              nir_intrinsic_load_barycentric_at_offset);
}

bool
ir3_nir_lower_load_barycentric_at_offset(nir_shader *shader)
{
   assert(shader->info.stage == MESA_SHADER_FRAGMENT);

   return nir_shader_lower_instructions(
      shader, ir3_nir_lower_load_barycentric_at_offset_filter,
      ir3_nir_lower_load_barycentric_at_offset_instr, NULL);
}
