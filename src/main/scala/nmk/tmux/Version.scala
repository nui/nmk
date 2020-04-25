package nmk.tmux

object Version {

  sealed abstract class Version(val v: String) extends Ordered[Version] {
    override def compare(that: Version): Int = allVersions.indexOf(this) - allVersions.indexOf(that)

    override def toString: String = v
  }

  case object V21 extends Version("2.1")

  case object V22 extends Version("2.2")

  case object V23 extends Version("2.3")

  case object V24 extends Version("2.4")

  case object V25 extends Version("2.5")

  case object V26 extends Version("2.6")

  case object V27 extends Version("2.7")

  case object V28 extends Version("2.8")

  case object V29 extends Version("2.9")

  case object V29a extends Version("2.9a")

  case object V30 extends Version("3.0")

  case object V30a extends Version("3.0a")

  case object V31 extends Version("3.1")

  private val allVersions = List(V21, V22, V23, V24, V25, V26, V27, V28, V29, V29a, V30, V30a, V31)

  private val unsupportedVersions = List(V24)

  private val supportedVersions = allVersions.filterNot(unsupportedVersions.contains)

  def supported: Iterator[Version] = supportedVersions.toIterator
}
