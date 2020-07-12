package nmk.tmux

object Version {

  sealed abstract class Version(val v: String) extends Ordered[Version] {
    override def compare(that: Version): Int = allVersions.indexOf(this) - allVersions.indexOf(that)

    override def toString: String = v
  }

  case object V26 extends Version("2.6")

  case object V27 extends Version("2.7")

  case object V28 extends Version("2.8")

  case object V29 extends Version("2.9")

  case object V29a extends Version("2.9a")

  case object V30 extends Version("3.0")

  case object V30a extends Version("3.0a")

  case object V31 extends Version("3.1")

  case object V31a extends Version("3.1a")

  case object V31b extends Version("3.1b")

  private val allVersions = List(V26, V27, V28, V29, V29a, V30, V30a, V31, V31a, V31b)

  private val unsupportedVersions = List()

  private val supportedVersions = allVersions.filterNot(unsupportedVersions.contains)

  def supported: Iterator[Version] = supportedVersions.iterator
}
