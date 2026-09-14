// SPDX-License-Identifier: GPL-3.0-only
package me.weishu.kernelsu.ui.util

import androidx.compose.material3.SnackbarHostState
import androidx.compose.runtime.compositionLocalOf

val LocalSnackbarHost = compositionLocalOf<SnackbarHostState> {
    error("SUSFS SnackbarHost is not provided")
}
