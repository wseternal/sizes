package cid.zhaohua.frontend.ui

import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Menu
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.async
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun TopNavBar(appContext: AppContext) {
    var searchText by remember { mutableStateOf("") }
    var isSearching by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()
    val snackbarHostState = appContext.snackbarHostState

    TopAppBar(
        title = {
            Surface(
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(8.dp),
                shadowElevation = 4.dp
            ) {
                OutlinedTextField(
                    value = searchText,
                    onValueChange = { searchText = it },
                    modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
                    placeholder = { Text("Search...") },
                    singleLine = true,
                    keyboardOptions = KeyboardOptions(imeAction = ImeAction.Search)
                )
            }
        },
        navigationIcon = {
            if (!appContext.showPermanentDrawer) IconButton(onClick = { scope.launch { appContext.drawerState.open() } }) {
                Icon(Icons.Default.Menu, contentDescription = "Menu")
            }
        },
        actions = {
            IconButton(enabled = searchText.isNotEmpty() && !isSearching, onClick = {
                isSearching = true
                scope.launch {
                    async {
                        snackbarHostState.showSnackbar(
                            message = """Search button clicked, try to search `$searchText`""",
                            withDismissAction = true
                        )
                    }.await()
                    isSearching = false
                }
            }) {
                Icon(Icons.Default.Search, contentDescription = "Search")
            }
        }
    )
}