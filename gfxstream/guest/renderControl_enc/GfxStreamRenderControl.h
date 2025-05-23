/*
 * Copyright 2024 Google LLC
 * SPDX-License-Identifier: Apache-2.0
 */

#ifndef GFXSTREAM_RENDER_CONTROL_H
#define GFXSTREAM_RENDER_CONTROL_H

#include <cstdint>

#include "GfxStreamConnectionManager.h"

GfxStreamTransportType renderControlGetTransport();

int32_t renderControlInit(GfxStreamConnectionManager* mgr, void* vkInfo);

#endif
