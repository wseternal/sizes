package cid.zhaohua.frontend.ui.components.jsontable

import kotlinx.serialization.json.JsonObject

enum class ColumnType(val type: String) {
    STRING("string"),
    BOOLEAN("boolean"),
    DOUBLE("double"),
    LONG("long"),
    ARRAY("array"),
    OBJECT("object"),
    // when deduce the type from JsonObject, `null` value yields this `UNKNOWN` type
    UNKNOWN("unknown")
}

data class ColumnConfig(
    val key: String,
    val label: String = key,
    val hiddenInCreation: Boolean = false,
    val type: ColumnType = ColumnType.STRING,
)

data class TableConfig(
    val columns: List<ColumnConfig>
)

data class TableData(
    val items: Collection<JsonObject>,
    val conf: TableConfig?
) {
    companion object {
        fun empty() = TableData(emptyList(), null)
    }
}

