package cid.zhaohua.frontend.ui.components.jsontable

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dataset
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme.colorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import cid.zhaohua.frontend.ui.TextCell
import kotlinx.serialization.json.*
import org.jetbrains.compose.ui.tooling.preview.Preview
import kotlin.math.max

@Composable
fun JsonTable(data: TableData) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(16.dp),
        colors = CardDefaults.cardColors(colorScheme.surface)
    ) {
        if (data.items.isEmpty()) {
            Row(
                horizontalArrangement = Arrangement.SpaceAround,
                verticalAlignment = Alignment.CenterVertically,
                modifier = Modifier.fillMaxWidth()
            ) {
                Icons.Default.Dataset
                TextCell(text = "Empty", isHeader = true)
            }
            return@Card
        }
        val conf = data.conf ?: configFromData(data.items.first())
        HeaderRow(data.items, conf)
        HorizontalDivider()
        DataRows(data.items, conf)
    }
}

@Composable
private fun DataRows(items: Collection<JsonObject>, config: TableConfig) {
    val weight = 1.0 / max(1, config.columns.size)
    items.forEach { row ->
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround
        ) {
            config.columns.forEach {
                TextCell(text = row[it.key]?.jsonPrimitive?.content ?: "", weight = weight.toFloat())
            }
        }
    }
}

@Composable
private fun HeaderRow(items: Collection<JsonObject>, config: TableConfig) {
    val weight  = 1.0 / max(1, config.columns.size)
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceAround
    ) {
        config.columns.forEach { config ->
            TextCell(text = config.label, isHeader = true, weight = weight.toFloat())
        }
    }
}

private fun configFromData(data: JsonObject): TableConfig {
    val columns = data.map { (k, v) ->
        val type = when (v) {
            is JsonArray -> ColumnType.ARRAY
            is JsonObject -> ColumnType.OBJECT
            is JsonPrimitive -> when {
                v.isString -> ColumnType.STRING
                v.booleanOrNull != null -> ColumnType.BOOLEAN
                v.doubleOrNull != null -> ColumnType.DOUBLE
                v.longOrNull != null -> ColumnType.LONG
                else -> throw IllegalStateException("unexpected value $v for $k")
            }

            is JsonNull -> ColumnType.UNKNOWN
        }
        ColumnConfig(key = k, type = type)
    }.sortedBy { it.key }
    return TableConfig(columns)
}

@Preview
@Composable
fun demo() {
    val params = """
    {
        "name": "some name here",
        "label": "some label",
        "path": "some path",
        "refresh_interval": 5
    }
    """.trimIndent()
    val jsonObject = Json.encodeToJsonElement(params).jsonObject
    JsonTable(TableData(listOf(jsonObject), null))
}