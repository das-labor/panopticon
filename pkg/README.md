# Creating a new release
When creating a new release of _Panopticon_, there are a few things that have to be considered beforehand. This small guide will help you through the process.

These are the steps of creating a release:

1. Bumping version numbers
1. Adding new sections to the changelogs
1. Building the project
1. Creating a tag
1. Writing a blog article containing a changelog

## Bumping version numbers
This means ***everywhere***:
 - in `.travis.yml` lines 112, 124
 - in `appveyor.yml` lines 1, 38
 - in `Cargo.toml` line 3
 - in `pkg/arch/PKGBUILD` lines 3, 26
 - in `pkg/osx/Info.plist` line 10
 - in `pkg/windows/package_zip.bat` line 7
 - in `qml/Title.qml` line 69

## Adding new sections to the changelogs
Several packages as well as the project itself contain a changelog. New entries have to be added to the following files:
 - `pkg/debian/debian/changelog`
 - `CHANGELOG`

## Building the project
To make sure that the version bump didn't corrupt the codebase, build the project as stated in the top level `README.md`.

## Creating a tag
If everything changed before is correctly committed, you now have to create a tag with the following format:
`<major>.<minor>.<patch>` e.g. `0.12.6`, `2.13.54`

After creating the tag it has to be pushed as well to make sure it can be referenced to in a changelog.

## Writing a blog article
After successfully creating a release you should consider writing a markdown formatted blog article containing a changelog and short summary of the changes.
The blog is a `jekyll` blog located here: [https://github.com/flanfly/panopticon.re](https://github.com/flanfly/panopticon.re).
To submit your article just add it to the other articles and open a pull request to get it merged.
