package dev.rossilorenzo.voxelrender

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.WindowInsets
import androidx.compose.foundation.layout.WindowInsetsSides
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.only
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.systemBars
import androidx.compose.foundation.layout.windowInsetsPadding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import dev.rossilorenzo.voxelrender.ui.theme.Voxel_rendererTheme

class ModelChooserActivity : ComponentActivity() {

    val getContent = registerForActivityResult(ActivityResultContracts.GetContent()) { uri: Uri? ->
        startActivity(Intent(this, ModelViewerActivity::class.java).apply {
            action = Intent.ACTION_VIEW
            data = uri
        })
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val ass = assets.list("example_models")!!.asList()
        val openAsset = { assetName: String ->
            startActivity(Intent(this, ModelViewerActivity::class.java).apply {
                action = Intent.ACTION_VIEW
                data = Uri.parse("assets:///example_models/$assetName")
            })
        }

        val exploreFiles = {
            getContent.launch("*/*")
        }

        setContent {
            Voxel_rendererTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    AssetList(ass, openAsset, exploreFiles)
                }
            }
        }
    }
}

@Composable
fun AssetList(assets: List<String>, openAsset: (String) -> Unit, pickFile: () -> Unit) {
    Scaffold (
        topBar = {
            Text(
                text = "Choose model",
                style = MaterialTheme.typography.headlineLarge,
                textAlign = TextAlign.Center,
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(0.dp, 15.dp),
            )
        }
    ) { paddingValues ->
        Column(
            Modifier
                .fillMaxSize()
                .padding(paddingValues)
                .windowInsetsPadding(
                    WindowInsets.systemBars.only(WindowInsetsSides.Horizontal)
                )) {
            LazyColumn(
                modifier = Modifier.fillMaxSize(),
                horizontalAlignment = Alignment.CenterHorizontally,
                verticalArrangement = Arrangement.Center,
            ) {
                for (asset in assets) {
                    item {
                        AssetButton(asset) {
                            openAsset(asset)
                        }

                        Spacer(
                            Modifier.height(20.dp)
                        )
                    }
                }
                item {
                    AssetButton("Explore") {
                        pickFile()
                    }
                }
            }
        }
    }
}

@Composable
fun AssetButton(name: String, onClick: () -> Unit) {
    Button(onClick) {
        Text(
            text = name,
            style = MaterialTheme.typography.headlineMedium,
            textAlign = TextAlign.Center,
        )
    }
}

@Preview(showBackground = true)
@Composable
fun AssetListPreview() {
    Voxel_rendererTheme {
        AssetList(listOf("First", "Second", "Third"), {}, {})
    }
}
