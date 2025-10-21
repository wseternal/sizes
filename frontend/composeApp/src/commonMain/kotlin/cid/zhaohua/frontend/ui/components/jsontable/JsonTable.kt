package cid.zhaohua.frontend.ui.components.jsontable

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Dataset
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme.colorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import cid.zhaohua.frontend.ui.TextCell
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.*
import org.jetbrains.compose.ui.tooling.preview.Preview

@Composable
fun JsonTable(data: TableData) {
    Card(
        modifier = Modifier.fillMaxWidth().padding(16.dp),
        colors = CardDefaults.cardColors(colorScheme.surface)
    ) {
        if (data.items.isEmpty()) {
            Table {
                Row {
                    Icons.Default.Dataset
                    TextCell(text = "Empty", isHeader = true)
                }
            }
            return@Card
        }
        val conf = data.conf ?: configFromData(data.items.first())
        Table {
            Row {
                conf.columns.forEach { config ->
                    TextCell(text = config.label, isHeader = true)
                }
            }
            data.items.forEach { row ->
                Row {
                    conf.columns.forEach {
                        TextCell(text = row[it.key]?.jsonPrimitive?.content ?: "")
                    }
                }
            }
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
fun Demo() {
    val params = """
    {
        "name": "some name here",
        "label": "some label",
        "path": "some path",
        "refresh_interval": 5
    }
    """.trimIndent()
    val jsonObject = Json.decodeFromString<JsonObject>(params)
    JsonTable(TableData(listOf(jsonObject), null))
}