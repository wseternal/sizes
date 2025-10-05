package cid.zhaohua.frontend.ui

import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp

@Composable
fun RowScope.TextCell(
    text: String,
    isHeader: Boolean = false,
    weight: Float? = null
) {
    Text(
        text = text,
        modifier = Modifier.padding(8.dp).apply {
            weight?.let { this.weight(it) }
        },
        fontWeight = if (isHeader) FontWeight.Bold else FontWeight.Normal
    )
}