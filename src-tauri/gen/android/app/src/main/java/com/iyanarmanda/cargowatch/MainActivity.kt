package com.iyanarmanda.cargowatch

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import com.google.firebase.messaging.FirebaseMessaging

class MainActivity : TauriActivity() {
  private external fun setFcmToken(token: String)

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)

    FirebaseMessaging.getInstance().token.addOnCompleteListener { task ->
      if (task.isSuccessful) {
        val token = task.result

        setFcmToken(token)
      }
    }
  }
}