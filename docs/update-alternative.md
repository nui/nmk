# Add new alternative group

```sh
# scala example
sudo update-alternatives \
    --install /usr/bin/scala  scala    /opt/scala-2.12.5/bin/scala 1 \
    --slave /usr/bin/fsc      fsc      /opt/scala-2.12.5/bin/fsc \
    --slave /usr/bin/scalac   scalac   /opt/scala-2.12.5/bin/scalac \
    --slave /usr/bin/scaladoc scaladoc /opt/scala-2.12.5/bin/scaladoc \
    --slave /usr/bin/scalap   scalap   /opt/scala-2.12.5/bin/scalap \
    --slave /usr/share/man/man1/fsc.1 fsc.1 /opt/scala-2.12.5/man/man1/fsc.1 \
    --slave /usr/share/man/man1/scala.1 scala.1 /opt/scala-2.12.5/man/man1/scala.1 \
    --slave /usr/share/man/man1/scalac.1 scalac.1 /opt/scala-2.12.5/man/man1/scalac.1 \
    --slave /usr/share/man/man1/scaladoc.1 scaladoc.1 /opt/scala-2.12.5/man/man1/scaladoc.1 \
    --slave /usr/share/man/man1/scalap.1 scalap.1 /opt/scala-2.12.5/man/man1/scalap.1
```

# Choose
`sudo update-alternatives --config scala`

