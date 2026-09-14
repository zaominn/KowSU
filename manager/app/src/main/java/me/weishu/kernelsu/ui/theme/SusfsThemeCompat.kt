// SPDX-License-Identifier: GPL-3.0-only
package me.weishu.kernelsu.ui.theme

import androidx.compose.runtime.Stable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color

/**
 * Small compatibility surface for the ReSukiSU SUSFS screen. KowSU does not
 * expose ReSukiSU's custom-background blur controls, so SUSFS cards follow the
 * active KowSU theme without adding a second theme preference system.
 */
@Stable
object CardConfig {
    const val cardAlpha: Float = 1f
}

@Stable
object ThemeConfig {
    const val isEnableBlur: Boolean = false
    const val isEnableBlurExp: Boolean = false
}

fun Modifier.blurEffect(): Modifier = this
fun Modifier.blurSource(): Modifier = this
fun Modifier.renderBackgroundBlur(tintColor: Color? = null): Modifier = this
