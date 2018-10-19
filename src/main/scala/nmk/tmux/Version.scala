package nmk.tmux

object Version {

  sealed abstract class Version(val v: Double) extends Ordered[Version] {
    override def compare(that: Version): Int = v.compare(that.v)

    override def toString: String = v.toString
  }

  case object V21 extends Version(2.1)

  case object V22 extends Version(2.2)

  case object V23 extends Version(2.3)

  case object V24 extends Version(2.4)

  case object V25 extends Version(2.5)

  case object V26 extends Version(2.6)

  case object V27 extends Version(2.7)

  case object V28 extends Version(2.8)

  def supported = Iterator(V21, V22, V23, V25, V26, V27, V28)

}
