package nmk

import java.nio.file.{Files, Paths}

import com.google.inject.Guice
import javax.inject.{Inject, Singleton}
import net.codingwell.scalaguice.InjectorExtensions._
import nmk.tmux.{Tmux, Version}

@Singleton
class Main @Inject()(tmux: Tmux,
                     zsh: Zsh) {
  private val NmkDir = scala.util.Properties.envOrNone("NMK_HOME").get
  private val ZshRc = Paths.get(NmkDir, "zsh", ".zshrc")

  def run = {
    Files.write(ZshRc, zsh.render(NmkDir).getBytes())
    Version.supported.foreach(v => {
      val conf = Paths.get(NmkDir, "tmux", s"$v.conf")
      Files.write(conf, tmux.render(v).getBytes())
    })
  }
}

object Main extends App {
  val injector = Guice.createInjector(new Module)
  injector.instance[Main].run
}
