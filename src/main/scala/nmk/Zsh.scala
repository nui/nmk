package nmk

import java.nio.file._

import javax.inject.Singleton

import scala.collection.mutable.ListBuffer

@Singleton
class Zsh {
  def render(nmkDir: String) = {
    val zshSrcDir = Paths.get(nmkDir, "zsh", "src", "zshrc")
    val paths = ListBuffer.empty[Path]
    Files.list(zshSrcDir).forEach(p => {
      paths.append(p)
    })
    paths.sorted.flatMap(p => Files.readAllLines(p).toArray()).mkString("\n")
  }
}
