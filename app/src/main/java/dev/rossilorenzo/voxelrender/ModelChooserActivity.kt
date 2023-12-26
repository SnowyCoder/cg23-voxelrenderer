package dev.rossilorenzo.voxelrender

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import dev.rossilorenzo.voxelrender.ui.theme.Voxel_rendererTheme

class ModelChooserActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val ass = assets.list("example_models")!!.asList()
        val openAsset = { assetName: String ->
            startActivity(Intent(this, ModelViewerActivity::class.java).apply {
                action = Intent.ACTION_VIEW
                data = Uri.parse("assets:///example_models/$assetName")
            })
        }

        setContent {
            Voxel_rendererTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    Greeting(ass, openAsset)
                }
            }
        }
    }
}

@Composable
fun Greeting(assets: List<String>, openAsset: (String) -> Unit) {
    Scaffold (
        topBar = {
            Text(
                text = "Choose example",
                style = MaterialTheme.typography.headlineLarge
            )
        }
    ) { paddingValues ->
        Column(Modifier.fillMaxSize().padding(paddingValues)) {
            LazyColumn(
                modifier = Modifier.fillMaxSize()
            ) {
                for (asset in assets)
                item {
                    Text(
                        text = asset,
                        modifier = Modifier.clickable {
                            Log.e("ASSET", "Clicked Asset $asset")
                            openAsset(asset)
                        },
                        style = MaterialTheme.typography.titleLarge
                    )
                }
            }
        }
    }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    Voxel_rendererTheme {
        Greeting(listOf("First", "Second", "Third")) {}
    }
}
