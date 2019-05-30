# Bullet

My Bullet-TODO assistant.

## What

This is what my TODO list looks like:

```
* Task
X Completed
> Migrated
< Scheduled
- Cancelled

## 2019-05-27
x Version-pin tooling for BE
x Build BE from scratch
* Replace Marco's deploy keys in the CI

## 2019-05-28
x Replace Marco's deploy keys in the CI
* Write new ticket: failed logins on CC
* Write new ticket: Create users for Kubectl

## 2019-05-29
x Write new ticket: Create users for Kubectl
> Write new ticket: failed logins on CC
```

Bullet reads from stdin and writes to stdout.

This first version adds today's date at the bottom, if it is not there already.

## How

Once `bullet` is in your `$PATH`, type in vim:

```
:%!bullet
```
