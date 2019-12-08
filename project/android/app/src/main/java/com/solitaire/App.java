package com.solitaire;

class App {
  private static native int go();

  int start_server() {
    return go();
  }
}
