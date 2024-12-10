package com.bookmark.app

import android.content.Intent
import android.os.Bundle

class MainActivity : TauriActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
    super.onCreate(savedInstanceState)
    // TODO: set the view to the page where the bookmark will be added
    // setContentView()

    // https://android.googlesource.com/platform/development/+/master/samples/ApiDemos/src/com/example/android/apis/content/ProcessText.java
    // TODO: parse the text, if it a url filed, set it as URL
    val text: CharSequence? = intent?.getCharSequenceExtra(Intent.EXTRA_PROCESS_TEXT)
    val formattedText = text.toString()
}

}