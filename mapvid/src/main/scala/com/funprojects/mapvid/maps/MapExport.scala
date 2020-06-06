package com.funprojects.mapvid.maps

import java.io.File
import java.net.URL

import javax.imageio.ImageIO



object MapExport {

  

  def construct_url(): String = {
    "https://api.mapbox.com/styles/v1/mapbox/streets-v11/static/-122.4241,37.78,14.25,0,60/600x600?access_token=pk.eyJ1IjoiZGF2aWRrb2UiLCJhIjoiY2thanhmbzdtMGYyZjJ5bW42bDVic2FhaiJ9.qQG2FOW6XniJKWml0705kw"
  }

  def download(): Boolean = {
    val url = new URL(construct_url())
    val img = ImageIO.read(url)
    val file = new File("C:\\Users\\koerveda\\Downloads\\downloaded.png");
    ImageIO.write(img, "png", file);
  }
}
