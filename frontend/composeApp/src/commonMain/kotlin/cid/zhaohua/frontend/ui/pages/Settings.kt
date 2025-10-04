package cid.zhaohua.frontend.ui.pages

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.wrapContentWidth
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme.colorScheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.snapshots.SnapshotStateList
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import cid.zhaohua.frontend.ktorfit.SizesApi
import cid.zhaohua.frontend.ktorfit.WatchDirectoryConfiguration
import cid.zhaohua.frontend.ui.AppContext
import cid.zhaohua.frontend.ui.TableCell
import kotlinx.coroutines.launch

@Composable
fun SettingsPage(appContext: AppContext) {
    val sizesApi = appContext.sizesApi
    val watchedDirs  = remember { mutableStateListOf<WatchDirectoryConfiguration>() }

    LaunchedEffect(Unit) {
        sizesApi.getWatchConfigurations().forEach {
            watchedDirs.add(it)
        }
    }

    Column {
        WatchedDirectoryTable(sizesApi, watchedDirs)
        HorizontalDivider()
    }
}

@Composable
private fun WatchedDirectoryTable(sizesApi: SizesApi, dirs: SnapshotStateList<WatchDirectoryConfiguration>) {
    Card(modifier = Modifier.fillMaxWidth().padding(16.dp),
        colors = CardDefaults.cardColors(colorScheme.surface)
    ) {
        TableHeader()
        HorizontalDivider(modifier = Modifier.padding(vertical = 4.dp))
        TableRows(dirs)
    }
}

@Composable
private fun TableRows(dirs: SnapshotStateList<WatchDirectoryConfiguration>) {
    val scope = rememberCoroutineScope()
    dirs.forEach {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceAround
        ) {
            TableCell(text = it.path, weight = 0.30f)
            TableCell(text = it.label, weight = 0.20f)
            TableCell(text = it.refreshInterval, weight = 0.10f)
            Row(
                modifier = Modifier.weight(0.40f),
                horizontalArrangement = Arrangement.Center
            ) {
                Button(
                    modifier = Modifier.weight(0.20f)
                        .wrapContentWidth(),
                    onClick = {
                        scope.launch {
                        }
                    }
                ) {
                    Text("Edit")
                }
            }
        }
    }
}

@Composable
private fun TableHeader() {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceAround
    ) {
        TableCell(text = "Path", isHeader = true, weight = 0.30f)
        TableCell(text = "Label", isHeader = true, weight = 0.20f)
        TableCell(text = "Refresh Interval", isHeader = true, weight = 0.10f)
        TableCell(text = "Action", isHeader = true, weight = 0.40f)
    }
}
