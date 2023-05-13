# Tutorial for starting

**Overview**

Also, the Hurd wiki probably can tell us everything, but it would be better if I can show some tutorials for beginners. More choices are good than only one.

**Topics**

1. Introduction
2. Setting up the development environment
3. Understanding the Hurd architecture
4. Creating a simple Hurd application
5. Contributing to the Hurd project
6. Conclusion

**Introduction**

The GNU Hurd project is free and open-source, but it is quite different from the open-source projects we have now. Here is a philosophy from the GNU Hurd Wiki.

{% embed url="https://www.gnu.org/philosophy/free-sw.html" %}
What is Free Software?
{% endembed %}

<mark style="color:green;">**Any contributions to the GNU Hurd project are welcome.**</mark>



**Source Code**

{% hint style="info" %}
The member of the project should download code by using the [Developer setup](https://savannah.gnu.org/maintenance/UsingGit/),

it can give you a write permission.
{% endhint %}

\[Here are all the repos of the GNU Hurd]\([https://savannah.gnu.org/git/?group=hurd](https://savannah.gnu.org/git/?group=hurd))

* Hurd meta package
  * It with README in this repo
  * `git clone https://git.savannah.gnu.org/git/hurd.git`
* glibc mainance
  * It with README, it shows you how to use glibc with [documentation](https://www.gnu.org/software/hurd/source\_repositories/glibc.html)&#x20;
  * `git clone https://git.savannah.gnu.org/git/hurd/glibc.git`
* GNU Mach
  * This repo use to save source code for GNU Mach
  * `git clone https://git.savannah.gnu.org/git/hurd/gnumach.git`
* Hurd
  * This repo use to save the source code for the Hurd
  * `git clone https://git.savannah.gnu.org/git/hurd/hurd.git`
* The great next stuff
  * It with README in this repo tell you \`incubator\`
  * `git clone https://git.savannah.gnu.org/git/hurd/incubator.git`
* POSIX threading library
  * It is for saving the source code for POSIX threading library
  * `git clone https://git.savannah.gnu.org/git/hurd/libpthread.git`
* MIG
  * It is for saving the source code for the Mach 3.0 INterface Generator
  * `git clone https://git.savannah.gnu.org/git/hurd/mig.git`
* procfs
  * It is for saving the source code for <mark style="color:red;">**procfs**</mark>
  * `git clone https://git.savannah.gnu.org/git/hurd/procfs.git`
* unionfs
  * It is for saving the source code for <mark style="color:red;">**unionfs tanslator**</mark> for the GNU Hurd
  * `git clone https://git.savannah.gnu.org/git/hurd/unionfs.git`
* Viengoos
  * It is for saving the source code for <mark style="color:red;">**Hurd on L4**</mark>
  * `git clone https://git.savannah.gnu.org/git/hurd/viengoos.git`
* Web pages
  * It is for saving the source code for Hurd's website
  * `git clone https://git.savannah.gnu.org/git/hurd/web.git`







**Reference**

[https://www.gnu.org/software/hurd/users-guide/using\_gnuhurd.html](https://www.gnu.org/software/hurd/users-guide/using\_gnuhurd.html)

