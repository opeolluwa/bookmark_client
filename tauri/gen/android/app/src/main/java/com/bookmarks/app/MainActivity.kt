package com.bookmarks.app

import android.content.Intent
import android.os.Bundle

class MainActivity : TauriActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // setContentView(R.layout.process_text_main);
        val text: CharSequence? = intent?.getCharSequenceExtra(Intent.EXTRA_PROCESS_TEXT)
        // Process the text as string to remove color formatting or styles
        val formattedText = text.toString()
    }
}

