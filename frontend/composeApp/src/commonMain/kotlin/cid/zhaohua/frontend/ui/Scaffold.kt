package cid.zhaohua.frontend.ui

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material3.*
import androidx.compose.material3.MaterialTheme.colorScheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun AppScaffold(appContext: AppContext) {
    val snackbarHostState = appContext.snackbarHostState
    val scope = rememberCoroutineScope()

    Scaffold(
        topBar = {
            TopNavBar(appContext)
        },
        floatingActionButton = {
            FloatingActionButton(onClick = {
                scope.launch {
                    snackbarHostState.showSnackbar(
                        message = "FAB button clicked",
                        withDismissAction = true
                    )
                }
            }) {
                Text("FAB")
            }
        },
        content = { innerPadding ->
            val pageSupplier = appContext.pageSuppliers[appContext.itemSelected?.key.orEmpty()]
            if(pageSupplier != null) {
                Box(modifier = Modifier.fillMaxSize()
                    .padding(innerPadding)) {
                    pageSupplier()
                }
            } else {
                Box (modifier = Modifier.fillMaxSize()
                    .padding(innerPadding),
                    contentAlignment = Alignment.Center

                ) {
                    Text(text = """Content for ${appContext.itemSelected?.label ?: "non-selected" }""")
                }
            }
        },
        snackbarHost = {
            SnackbarHost(hostState = snackbarHostState) {
                Snackbar(it, modifier = Modifier.widthIn(max = 300.dp), containerColor = colorScheme.primary)
            }
        }
    )
}