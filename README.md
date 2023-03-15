# FodlerEyez
(as in toddler :baby:, and :eye:s)

### Welcome to the last text UI, folder and file size utility, for gamers, on Windows, with sarcastic comments, you will ever need (probably).

![Example output from fodlereyez.exe, showing the contents of a c:\ drive, listing each
entry under the root, alongside an optional comment and the size of the entry. The top
entry is dark red with smaller directories and files underneath it coloured from orange to
yellow, green and blue with decreasing size. Entries which cannot be opened are greyed out.
There are instructions in magenta on the first line.](demo1.png)

- [x] Fodlereyez is colourful!

- [x] Fodlereyez is helpful!

- [x] Fodlereyez is a text based UI app, that shows your files and folders sorted by size, and
  coloured depending on their absolute size. So directories containing lots of data, or large
  files, are red, and small files, or directories containing little data, are white/blue. (Because
  Windows FileExplorer doesn't show directory size in a column; you have to open each individual
  directory's properties)

- [x] Fodlereyez also has helpful / annoying / cringe comments on your folders and files, as
  shown in the
  image above.

### Instructions
Get it [here](/releases/latest)

* You invoke Fodlereyez from a command prompt, passing the directory you want to start from as the
  argument. E.g.

```
c:\Users\james>fodlereyez.exe c:\
```

If you don't provide a folder argument, it will default to the current folder.

* You can turn off comments, and show hidden files and folders with either command line arguments
  or ui commands as shown in the image below:

![The same listing as image 1 above, but with no comments shown and hidden files and folders
appearing in the listing.](demo2.png)

* You can drill down into subdirectories by left-clicking their name, or by selecting them using
  the arrow keys and pressing [Enter].

* Pressing [Space] with an entry highlighted will open that entry in FileExplorer, so that you can
  make changes, such as deleting unnecessary files or folders.

* [Esc] will exit the app.

Take a look at available command line options using `--help`

```
c:\Users\james>fodlereyez.exe --help
```

![A command prompt windows terminal, showing the output from the --help flag on running
fodlereyes.exe](demo3.png)
(yes, I know I misspelled click as clilck)

If you are interested, there is a readme about the dev process and why I made this project
[here](DEV_README.md)
