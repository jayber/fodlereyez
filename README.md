# FodlerEyez
(as in toddler :baby:, and :eye:s)
[download binary](https://github.com/jayber/fodlereyez/releases/latest)

### Welcome to the last text UI, folder and file size utility, for gamers, on Windows and Linux, with sarcastic comments, you will ever need.

![Example output from fodlereyez.exe, showing the contents of a c:\ drive, listing each
entry under the root, alongside an optional comment and the size of the entry. The top
entry is dark red with smaller directories and files underneath it, coloured from orange to
yellow, green and blue with decreasing size. Entries which cannot be opened are greyed out.
There are instructions in magenta on the first line.](demo1.png)

- [x] Fodlereyez is colourful!

- [x] Fodlereyez is helpful!

- [x] Fodlereyez is a text based UI app, that shows your files and folders sorted by relative size,
  and coloured depending on their absolute size. So directories containing lots of data, or large
  files, are red, and small files, or directories containing little data, are white/blue.

- [x] Fodlereyez also has helpful / annoying / cringe comments on your folders and files, as
  shown in the image above (press "c" to hide).

Want it to do something else? Create
an [issue or report  bug](https://github.com/jayber/fodlereyez/issues).

### Instructions

Get it [here](https://github.com/jayber/fodlereyez/releases/latest)

* You invoke Fodlereyez from a command prompt, passing the directory you want to start from as the
  argument. E.g.

```
c:\Users\james>fodlereyez.exe c:\
```

If you don't provide a folder argument, it will default to the current folder.

![An alternative listing shown on a terminal with dark grey background of a linux filesystem
showing the .../SteamLibrary/steamapps/common directory. Entries have comments containing
light-hearted descriptions of the games they contain.](demo4.png)

* You can turn off comments, and show hidden files and folders with either command line arguments
  or ui commands.

* You can drill down into subdirectories by left-clicking their name, or by selecting them using
  the arrow keys and pressing [Enter].

* Pressing [Space] with an entry highlighted will open that entry in either file `explorer` on
  Windows, or `gnome-terminal` otherwise, so that you can
  make changes, such as deleting unnecessary files or folders.

* [Esc] will exit the app.

* Does not traverse symlinks, and excludes certain directories on linux containing virtual files
  or mount points (see final column in image above). However, you can still see the contents of
  these entries by invoking the app with their path as the argument.

![An alternative listing shown on a terminal with purple background of an wsl/Ubuntu filesystem
root. In the size column on the right, some entries show the text "-link-" or "-excl-" instead of
a size](demo2.png)

Take a look at available command line options using `--help`

```
c:\Users\james>fodlereyez.exe --help
```

![A command prompt windows terminal, showing the output from the --help flag on running
fodlereyes.exe](demo3.png)

If you are interested, there is a readme about the dev process and why I made this project
[here](DEV_README.md)
