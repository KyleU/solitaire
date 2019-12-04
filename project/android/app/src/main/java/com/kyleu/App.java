package com.solitaire;

public class App {
  private static native int go();

  public int start_server() {
    return go();
  }
}
