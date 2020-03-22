package nmk

import java.nio.file._

import javax.inject.Singleton

import scala.jdk.CollectionConverters._
import scala.collection.mutable.ListBuffer

@Singleton
class Zsh {
  def render(nmkDir: String): String = {
    val zshSrcDir = Paths.get(nmkDir, "zsh", "src", "zshrc")
    val paths = ListBuffer.empty[Path]
    Files.list(zshSrcDir).forEach(paths.append(_))
    paths.sorted.flatMap(Files.readAllLines(_).asScala).mkString("\n")
  }
}
