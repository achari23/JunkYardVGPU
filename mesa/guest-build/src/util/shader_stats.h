#ifndef __SHADER_STATS_H
#define __SHADER_STATS_H
#include <stdio.h>
#include <stdint.h>
#include "util/u_debug.h"

struct adreno_stats {
   int16_t maxwaves;
   int32_t inst;
   int32_t code_size;
   int32_t nops;
   int32_t mov;
   int32_t cov;
   int16_t full;
   int16_t half;
   int32_t last_baryf;
   int32_t last_helper;
   int32_t ss;
   int32_t sy;
   int32_t ss_stall;
   int32_t sy_stall;
   int32_t cat[8];
   int32_t stps;
   int32_t ldps;
   int32_t preamble_inst;
   bool early_preamble;
};

static inline int
adreno_stats_fprintf(FILE *fp, const char *prefix, const struct adreno_stats *stats)
{
   return fprintf(fp, "%s shader: %u maxwaves, %u inst, %u code size, %u nops, %u mov, %u cov, %u full, %u half, %u last-baryf, %u last-helper, %u (ss), %u (sy), %u (ss)-stall, %u (sy)-stall, %u cat0, %u cat1, %u cat2, %u cat3, %u cat4, %u cat5, %u cat6, %u cat7, %u stps, %u ldps, %u preamble inst, %u early-preamble\n", prefix, stats->maxwaves, stats->inst, stats->code_size, stats->nops, stats->mov, stats->cov, stats->full, stats->half, stats->last_baryf, stats->last_helper, stats->ss, stats->sy, stats->ss_stall, stats->sy_stall, stats->cat[0], stats->cat[1], stats->cat[2], stats->cat[3], stats->cat[4], stats->cat[5], stats->cat[6], stats->cat[7], stats->stps, stats->ldps, stats->preamble_inst, stats->early_preamble);
}

static inline void
adreno_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct adreno_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u maxwaves, %u inst, %u code size, %u nops, %u mov, %u cov, %u full, %u half, %u last-baryf, %u last-helper, %u (ss), %u (sy), %u (ss)-stall, %u (sy)-stall, %u cat0, %u cat1, %u cat2, %u cat3, %u cat4, %u cat5, %u cat6, %u cat7, %u stps, %u ldps, %u preamble inst, %u early-preamble", prefix, stats->maxwaves, stats->inst, stats->code_size, stats->nops, stats->mov, stats->cov, stats->full, stats->half, stats->last_baryf, stats->last_helper, stats->ss, stats->sy, stats->ss_stall, stats->sy_stall, stats->cat[0], stats->cat[1], stats->cat[2], stats->cat[3], stats->cat[4], stats->cat[5], stats->cat[6], stats->cat[7], stats->stps, stats->ldps, stats->preamble_inst, stats->early_preamble);
}

#define vk_add_adreno_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Max Waves Per Core", "Maximum number of simultaneous waves per core.", (stats)->maxwaves); \
   vk_add_exec_statistic_u64(out, "Instruction Count", "Total number of IR3 instructions in the final generated shader executable.", (stats)->inst); \
   vk_add_exec_statistic_u64(out, "Code size", "Total number of dwords in the final generated shader executable.", (stats)->code_size); \
   vk_add_exec_statistic_u64(out, "NOPs Count", "Number of NOP instructions in the final generated shader executable.", (stats)->nops); \
   vk_add_exec_statistic_u64(out, "MOV Count", "Number of MOV instructions in the final generated shader executable.", (stats)->mov); \
   vk_add_exec_statistic_u64(out, "COV Count", "Number of COV instructions in the final generated shader executable.", (stats)->cov); \
   vk_add_exec_statistic_u64(out, "Registers used", "Number of registers used in the final generated shader executable.", (stats)->full); \
   vk_add_exec_statistic_u64(out, "Half-registers used", "Number of half-registers used in the final generated shader executable.", (stats)->half); \
   vk_add_exec_statistic_u64(out, "Last interpolation instruction", "The instruction where varying storage in Local Memory is released", (stats)->last_baryf); \
   vk_add_exec_statistic_u64(out, "Last helper instruction", "The instruction where helper invocations are killed", (stats)->last_helper); \
   vk_add_exec_statistic_u64(out, "Instructions with SS sync bit", "SS bit is set for instructions which depend on a result of long instructions to prevent RAW hazard.", (stats)->ss); \
   vk_add_exec_statistic_u64(out, "Instructions with SY sync bit", "SY bit is set for instructions which depend on a result of loads from global memory to prevent RAW hazard.", (stats)->sy); \
   vk_add_exec_statistic_u64(out, "Estimated cycles stalled on SS", "A better metric to estimate the impact of SS syncs.", (stats)->ss_stall); \
   vk_add_exec_statistic_u64(out, "Estimated cycles stalled on SY", "A better metric to estimate the impact of SY syncs.", (stats)->sy_stall); \
   vk_add_exec_statistic_u64(out, "cat0 instructions", "Number of cat0 instructions.", (stats)->cat[0]); \
   vk_add_exec_statistic_u64(out, "cat1 instructions", "Number of cat1 instructions.", (stats)->cat[1]); \
   vk_add_exec_statistic_u64(out, "cat2 instructions", "Number of cat2 instructions.", (stats)->cat[2]); \
   vk_add_exec_statistic_u64(out, "cat3 instructions", "Number of cat3 instructions.", (stats)->cat[3]); \
   vk_add_exec_statistic_u64(out, "cat4 instructions", "Number of cat4 instructions.", (stats)->cat[4]); \
   vk_add_exec_statistic_u64(out, "cat5 instructions", "Number of cat5 instructions.", (stats)->cat[5]); \
   vk_add_exec_statistic_u64(out, "cat6 instructions", "Number of cat6 instructions.", (stats)->cat[6]); \
   vk_add_exec_statistic_u64(out, "cat7 instructions", "Number of cat7 instructions.", (stats)->cat[7]); \
   vk_add_exec_statistic_u64(out, "STP Count", "Number of STore Private instructions in the final generated shader executable.", (stats)->stps); \
   vk_add_exec_statistic_u64(out, "LDP Count", "Number of LoaD Private instructions in the final generated shader executable.", (stats)->ldps); \
   vk_add_exec_statistic_u64(out, "Preamble Instruction Count", "Total number of IR3 instructions in the preamble.", (stats)->preamble_inst); \
   vk_add_exec_statistic_bool(out, "Early preamble", "Whether the preamble will be executed early.", (stats)->early_preamble); \
} while(0)

struct agx2_stats {
   int32_t instrs;
   int32_t alu;
   int32_t fscib;
   int32_t ic;
   int32_t code_size;
   int16_t gprs;
   int16_t uniforms;
   int32_t scratch;
   int16_t threads;
   int32_t loops;
   int32_t spills;
   int32_t fills;
};

static inline int
agx2_stats_fprintf(FILE *fp, const char *prefix, const struct agx2_stats *stats)
{
   return fprintf(fp, "%s shader: %u instrs, %u alu, %u fscib, %u ic, %u code size, %u gprs, %u uniforms, %u scratch, %u threads, %u loops, %u:%u spills:fills\n", prefix, stats->instrs, stats->alu, stats->fscib, stats->ic, stats->code_size, stats->gprs, stats->uniforms, stats->scratch, stats->threads, stats->loops, stats->spills, stats->fills);
}

static inline void
agx2_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct agx2_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u instrs, %u alu, %u fscib, %u ic, %u code size, %u gprs, %u uniforms, %u scratch, %u threads, %u loops, %u:%u spills:fills", prefix, stats->instrs, stats->alu, stats->fscib, stats->ic, stats->code_size, stats->gprs, stats->uniforms, stats->scratch, stats->threads, stats->loops, stats->spills, stats->fills);
}

#define vk_add_agx2_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Instructions", "Instruction count", (stats)->instrs); \
   vk_add_exec_statistic_u64(out, "ALU", "Estimated ALU cycle count", (stats)->alu); \
   vk_add_exec_statistic_u64(out, "FSCIB", "Estimated F16/F32/SCIB cycle count", (stats)->fscib); \
   vk_add_exec_statistic_u64(out, "IC", "Estimated IC cycle count", (stats)->ic); \
   vk_add_exec_statistic_u64(out, "Code size", "Binary size in bytes", (stats)->code_size); \
   vk_add_exec_statistic_u64(out, "GPRs", "Number of 16-bit GPRs", (stats)->gprs); \
   vk_add_exec_statistic_u64(out, "Uniforms", "Number of 16-bit uniform registers", (stats)->uniforms); \
   vk_add_exec_statistic_u64(out, "Scratch", "Scratch size per thread in bytes", (stats)->scratch); \
   vk_add_exec_statistic_u64(out, "Threads", "Maximum number of threads in flight on a compute unit", (stats)->threads); \
   vk_add_exec_statistic_u64(out, "Loops", "Number of hardware loops", (stats)->loops); \
   vk_add_exec_statistic_u64(out, "Spills", "Number of spill (stack store) instructions", (stats)->spills); \
   vk_add_exec_statistic_u64(out, "Fills", "Number of fill (stack load) instructions", (stats)->fills); \
} while(0)

struct midgard_stats {
   int32_t inst;
   int32_t bundles;
   int16_t registers;
   int16_t threads;
   int32_t quadwords;
   int32_t loops;
   int32_t spills;
   int32_t fills;
};

static inline int
midgard_stats_fprintf(FILE *fp, const char *prefix, const struct midgard_stats *stats)
{
   return fprintf(fp, "%s shader: %u inst, %u bundles, %u registers, %u threads, %u quadwords, %u loops, %u:%u spills:fills\n", prefix, stats->inst, stats->bundles, stats->registers, stats->threads, stats->quadwords, stats->loops, stats->spills, stats->fills);
}

static inline void
midgard_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct midgard_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u inst, %u bundles, %u registers, %u threads, %u quadwords, %u loops, %u:%u spills:fills", prefix, stats->inst, stats->bundles, stats->registers, stats->threads, stats->quadwords, stats->loops, stats->spills, stats->fills);
}

#define vk_add_midgard_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Instructions", "Instruction count", (stats)->inst); \
   vk_add_exec_statistic_u64(out, "Bundles", "Instruction bundles", (stats)->bundles); \
   vk_add_exec_statistic_u64(out, "Registers", "Register usage in vec4s", (stats)->registers); \
   vk_add_exec_statistic_u64(out, "Threads", "Maximum number of threads in flight on a compute unit", (stats)->threads); \
   vk_add_exec_statistic_u64(out, "Quadwords", "Binary size in quadwords", (stats)->quadwords); \
   vk_add_exec_statistic_u64(out, "Loops", "Number of hardware loops", (stats)->loops); \
   vk_add_exec_statistic_u64(out, "Spills", "Number of spill instructions", (stats)->spills); \
   vk_add_exec_statistic_u64(out, "Fills", "Number of fill instructions", (stats)->fills); \
} while(0)

struct bifrost_stats {
   int32_t instrs;
   int32_t tuples;
   int32_t clauses;
   float cycles;
   float arith;
   float t;
   float ldst;
   float v;
   int16_t preloads;
   int16_t threads;
   int32_t code_size;
   int32_t loops;
   int32_t spills;
   int32_t fills;
};

static inline int
bifrost_stats_fprintf(FILE *fp, const char *prefix, const struct bifrost_stats *stats)
{
   return fprintf(fp, "%s shader: %u instrs, %u tuples, %u clauses, %f cycles, %f arith, %f t, %f ldst, %f v, %u preloads, %u threads, %u code size, %u loops, %u:%u spills:fills\n", prefix, stats->instrs, stats->tuples, stats->clauses, stats->cycles, stats->arith, stats->t, stats->ldst, stats->v, stats->preloads, stats->threads, stats->code_size, stats->loops, stats->spills, stats->fills);
}

static inline void
bifrost_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct bifrost_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u instrs, %u tuples, %u clauses, %f cycles, %f arith, %f t, %f ldst, %f v, %u preloads, %u threads, %u code size, %u loops, %u:%u spills:fills", prefix, stats->instrs, stats->tuples, stats->clauses, stats->cycles, stats->arith, stats->t, stats->ldst, stats->v, stats->preloads, stats->threads, stats->code_size, stats->loops, stats->spills, stats->fills);
}

#define vk_add_bifrost_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Instructions", "Instruction count", (stats)->instrs); \
   vk_add_exec_statistic_u64(out, "Tuples", "Tuple count", (stats)->tuples); \
   vk_add_exec_statistic_u64(out, "Clauses", "Clause count", (stats)->clauses); \
   vk_add_exec_statistic_f64(out, "Cycles", "Estimated normalized cycles", (stats)->cycles); \
   vk_add_exec_statistic_f64(out, "Arithmetic", "Estimated normalized arithmetic cycles", (stats)->arith); \
   vk_add_exec_statistic_f64(out, "Texture", "Estimated normalized Texture cycles", (stats)->t); \
   vk_add_exec_statistic_f64(out, "Load/store", "Estimated normalized Load/Store cycles", (stats)->ldst); \
   vk_add_exec_statistic_f64(out, "Varying", "Estimated normalized Varying cycles", (stats)->v); \
   vk_add_exec_statistic_u64(out, "Preloads", "Preload count", (stats)->preloads); \
   vk_add_exec_statistic_u64(out, "Threads", "Maximum number of threads in flight on a compute unit", (stats)->threads); \
   vk_add_exec_statistic_u64(out, "Code size", "Binary size in bytes", (stats)->code_size); \
   vk_add_exec_statistic_u64(out, "Loops", "Number of hardware loops", (stats)->loops); \
   vk_add_exec_statistic_u64(out, "Spills", "Number of spill instructions", (stats)->spills); \
   vk_add_exec_statistic_u64(out, "Fills", "Number of fill instructions", (stats)->fills); \
} while(0)

struct valhall_stats {
   int32_t instrs;
   float cycles;
   float fma;
   float cvt;
   float sfu;
   float v;
   float t;
   float ls;
   int32_t code_size;
   int16_t threads;
   int32_t loops;
   int32_t spills;
   int32_t fills;
};

static inline int
valhall_stats_fprintf(FILE *fp, const char *prefix, const struct valhall_stats *stats)
{
   return fprintf(fp, "%s shader: %u instrs, %f cycles, %f fma, %f cvt, %f sfu, %f v, %f t, %f ls, %u code size, %u threads, %u loops, %u:%u spills:fills\n", prefix, stats->instrs, stats->cycles, stats->fma, stats->cvt, stats->sfu, stats->v, stats->t, stats->ls, stats->code_size, stats->threads, stats->loops, stats->spills, stats->fills);
}

static inline void
valhall_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct valhall_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u instrs, %f cycles, %f fma, %f cvt, %f sfu, %f v, %f t, %f ls, %u code size, %u threads, %u loops, %u:%u spills:fills", prefix, stats->instrs, stats->cycles, stats->fma, stats->cvt, stats->sfu, stats->v, stats->t, stats->ls, stats->code_size, stats->threads, stats->loops, stats->spills, stats->fills);
}

#define vk_add_valhall_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Instructions", "Instruction count", (stats)->instrs); \
   vk_add_exec_statistic_f64(out, "Cycles", "Estimated normalized cycles", (stats)->cycles); \
   vk_add_exec_statistic_f64(out, "FMA", "Estimated normalized FMA (Fused Multiply-Add) cycles", (stats)->fma); \
   vk_add_exec_statistic_f64(out, "CVT", "Estimated normalized CVT (ConVerT) cycles", (stats)->cvt); \
   vk_add_exec_statistic_f64(out, "SFU", "Estimated normalized SFU (Special Function Unit) cycles", (stats)->sfu); \
   vk_add_exec_statistic_f64(out, "Varying", "Estimated normalized Varying cycles", (stats)->v); \
   vk_add_exec_statistic_f64(out, "Texture", "Estimated normalized Texture cycles", (stats)->t); \
   vk_add_exec_statistic_f64(out, "Load/store", "Estimated normalized Load/Store cycles", (stats)->ls); \
   vk_add_exec_statistic_u64(out, "Code size", "Binary size in bytes", (stats)->code_size); \
   vk_add_exec_statistic_u64(out, "Threads", "Maximum number of threads in flight on a compute unit", (stats)->threads); \
   vk_add_exec_statistic_u64(out, "Loops", "Number of hardware loops", (stats)->loops); \
   vk_add_exec_statistic_u64(out, "Spills", "Number of spill instructions", (stats)->spills); \
   vk_add_exec_statistic_u64(out, "Fills", "Number of fill instructions", (stats)->fills); \
} while(0)

struct videocore_vi_stats {
   int32_t instrs;
   int32_t thread_count;
   int32_t spill_size;
   int32_t spills;
   int32_t fills;
   int32_t read_stalls;
};

static inline int
videocore_vi_stats_fprintf(FILE *fp, const char *prefix, const struct videocore_vi_stats *stats)
{
   return fprintf(fp, "%s shader: %u instrs, %u thread count, %u spill size, %u:%u spills:fills, %u read stalls\n", prefix, stats->instrs, stats->thread_count, stats->spill_size, stats->spills, stats->fills, stats->read_stalls);
}

static inline void
videocore_vi_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct videocore_vi_stats *stats)
{
   util_debug_message(debug, SHADER_INFO, "%s shader: %u instrs, %u thread count, %u spill size, %u:%u spills:fills, %u read stalls", prefix, stats->instrs, stats->thread_count, stats->spill_size, stats->spills, stats->fills, stats->read_stalls);
}

#define vk_add_videocore_vi_stats(out, stats) do { \
   vk_add_exec_statistic_u64(out, "Instruction Count", "Number of QPU instructions", (stats)->instrs); \
   vk_add_exec_statistic_u64(out, "Thread Count", "Number of QPU threads dispatched", (stats)->thread_count); \
   vk_add_exec_statistic_u64(out, "Spill Size", "Size of the spill buffer in bytes", (stats)->spill_size); \
   vk_add_exec_statistic_u64(out, "TMU Spills", "Number of times a register was spilled to memory", (stats)->spills); \
   vk_add_exec_statistic_u64(out, "TMU Fills", "Number of times a register was filled from memory", (stats)->fills); \
   vk_add_exec_statistic_u64(out, "QPU Read Stalls", "Number of cycles the QPU stalls for a register read dependency", (stats)->read_stalls); \
} while(0)


enum panfrost_stat_isa {
   PANFROST_STAT_MIDGARD,
   PANFROST_STAT_BIFROST,
   PANFROST_STAT_VALHALL,
};

struct panfrost_stats {
   enum panfrost_stat_isa isa;
   union {
      struct midgard_stats midgard;
      struct bifrost_stats bifrost;
      struct valhall_stats valhall;
   };
};

#define vk_add_panfrost_stats(out, stats) do { \
    if ((stats)->isa == PANFROST_STAT_MIDGARD) \
       vk_add_midgard_stats(out, &(stats)->midgard); \
    if ((stats)->isa == PANFROST_STAT_BIFROST) \
       vk_add_bifrost_stats(out, &(stats)->bifrost); \
    if ((stats)->isa == PANFROST_STAT_VALHALL) \
       vk_add_valhall_stats(out, &(stats)->valhall); \
} while(0)

static inline void
panfrost_stats_fprintf(FILE *fp, const char *prefix, const struct panfrost_stats *stats)
{
    if (stats->isa == PANFROST_STAT_MIDGARD)
       midgard_stats_fprintf(fp, prefix, &stats->midgard);
    if (stats->isa == PANFROST_STAT_BIFROST)
       bifrost_stats_fprintf(fp, prefix, &stats->bifrost);
    if (stats->isa == PANFROST_STAT_VALHALL)
       valhall_stats_fprintf(fp, prefix, &stats->valhall);
}

static inline void
panfrost_stats_util_debug(struct util_debug_callback *debug, const char *prefix, const struct panfrost_stats *stats)
{
    if (stats->isa == PANFROST_STAT_MIDGARD)
       midgard_stats_util_debug(debug, prefix, &stats->midgard);
    if (stats->isa == PANFROST_STAT_BIFROST)
       bifrost_stats_util_debug(debug, prefix, &stats->bifrost);
    if (stats->isa == PANFROST_STAT_VALHALL)
       valhall_stats_util_debug(debug, prefix, &stats->valhall);
}


#endif

