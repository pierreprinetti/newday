# Bullet

A Bullet-journal assistant.

Useful if you keep a TODO list in the form of an electronic bullet-journal.

## What

Given this journal:

```
+--------------+
| * Task       |
| x Completed  |
| > Migrated   |
| - Cancelled  |
+--------------+


## 2019-05-27
x Version-pin deploy tooling
x Build auth package
* Replace Marco's deploy keys in the CI

## 2019-05-28
x Replace Marco's deploy keys in the CI
* Write new ticket: failed logins on STG
* Write new ticket: Create users for Kubectl
* Investigate bug #123

## 2019-05-29
x Write new ticket: Create users for Kubectl
> Write new ticket: failed logins on STG
```

If the journal does not have an entry for today, Bullet will create it and report unfinished tasks.

```
## 2019-05-31
> Investigate bug #123
> Write new ticket: failed logins on STG
```

## How

Bullet reads from stdin and writes to stdout.

With `bullet` in your `$PATH`, type in vim:

```
:%!bullet
```
