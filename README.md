# RepoX

("RePox" is also fine.)

A tool to do all your CLI dev things without needing to remember the specific commands for each type of project.

## I'm Sorry, What?

Do you ever get tired of having to type `cargo build` in one place, and then `npm build` in another; and then `javac -classpath . app/Main.java` in yet another; or `python -m build` in another; and then `make` in another?

What if you have one tool that just... built your thing regardless of what KIND of thing it is?

...Anyone? No?

Look, I get it. This is a stupid idea. For most people. But not for me. I can't explain it. I just want it, and I've kept wanting it long enough, that I decided to start making it.

So anyway, I get tired of remembering all the various specific ways to do something in any repo I'm using. So I wrote this tool as a way to configure All The Projects(tm). It tries to detect whether you're in like... a Cmake project, or an NPM project, or a Cargo project or whatever, and then makes all the basic commands available to you.

With this tool, I can write a YAML config file once to describe each project type, and then expose a bunch of commands as needed. If your tool is wicked simple, then fine, you don't need this.

If your tool is NOT simple, then maybe something like this helps. I happen to work on some tools that just aren't as simple as I want them, but they really COULD be. Hence, this stupid tool that people who aren't me probably won't want.

## FAQ

1. Aren't you just lazy?

Yes. I am. Lazy as heck.

2. Aren't most project types' commands basically the same? Build, run, etc...?

Yes. But not all. And as per #1, I am lazy as heck.

3. Isn't writing this MORE work than just learning the tools?

Obviously, yes. Gosh, you're tedious.

4. But aren't YOU the one asking these FAQs?

...

Shut up.
