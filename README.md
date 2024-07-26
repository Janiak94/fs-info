# Fs-info

## WIP

This project is still a work in progress.

## Intended usage

This project is intended to be used as a utility for automatically polling remote machines virtual file systems about available space and inode usage and report if space and inodes are about to run out.

In addition it should poll to see which users use up the most space on the main file system in case it is about to run out of space.

The results will be reported by a slack bot in case space or inodes are about to run out.

I'm also considering implementing a REST API to make the results always available. To avoid over-polling a database should also be used to store the information.

## TODO list

- [ ] Configuration
- [x] VFS polling
- [ ] User directory polling
- [ ] Slack integration
- [ ] (REST API)
- [ ] (Database integration)