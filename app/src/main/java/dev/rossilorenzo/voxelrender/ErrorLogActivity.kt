package dev.rossilorenzo.voxelrender

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import dev.rossilorenzo.voxelrender.ui.theme.Voxel_rendererTheme

class ErrorLogActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val displayError = intent.extras?.getString("error") ?: "Unknown error"
        Log.e(TAG, "ERROR: $displayError")
        setContent {
            Voxel_rendererTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
                ) {
                    ErrorDisplay(displayError)
                }
            }
        }
    }
    companion object {
        const val TAG = "ErrorLog"
    }
}

@Composable
fun ErrorDisplay(error: String) {
    Column(
        Modifier.background(MaterialTheme.colorScheme.errorContainer).fillMaxSize(),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        Text(
            text = error,
            style = MaterialTheme.typography.titleLarge,
            color = MaterialTheme.colorScheme.error,
        )
    }
}

@Preview(showBackground = true)
@Composable
fun ErrorPreview() {
    Voxel_rendererTheme {
        ErrorDisplay("Cannot parse model\n\nCaused by:\n\tCannot determine format")
    }
}