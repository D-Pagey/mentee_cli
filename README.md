<a href="https://mentorcruise.com/mentor/danpage/">
<img src="https://cdn.mentorcruise.com/img/banner/navy-sm.svg" width="240" alt="MentorCruise">
</a>

# Mentee CLI

**Mentee CLI** is a lightweight, Rust-powered command-line interface for managing your mentees and tracking interactions with them — all backed by a local SQLite database.

It supports full CRUD operations on mentees, as well as their associated calls, video analyses, and payments. This tool is ideal for coaches, mentors, or anyone managing ongoing client or mentee relationships.

![Mentee CLI demo](./images/demo.gif)

---

## ✨ Features

- 📋 **List, add, update, delete** mentees
- 🔍 View detailed information about a mentee, including:
- 📈 Total calls, payments, and remaining call balance
- 📞 **Track calls** with dates, notes, and whether they were free
- 📹 **Log video analyses** with date, length, and notes
- 💰 **Record payments** and view associated stats
- 📊 Run summary calculations (e.g. total net, average net per call)
- 🗃️ Powered by SQLite — your data stays local and portable

---

## 🚀 Usage

```bash
mentees <COMMAND>
```

## 📖 Available Commands

| Command     | Description                               |
|-------------|-------------------------------------------|
| `list`      | List all mentees                          |
| `view`      | View more details of a mentee             |
| `add`       | Add a new mentee                          |
| `update`    | Update an existing mentee                 |
| `delete`    | Delete a mentee                           |
| `count`     | Count or sum columns across all mentees   |
| `calls`     | Manage mentee calls                       |
| `videos`    | Manage video analyses                     |
| `payments`  | Manage payments                           |
| `help`      | Show command help                         |


## 📊 Stats and Summaries

You can use the `count` command to quickly see high-level metrics like:
- Total number of mentees
- Total gross or net payments
- Total number of calls
- Average net per call (automatically calculated)

Example:

```bash
mentees count gross
```

## 🛠️ Installation

You can install the CLI via Homebrew:

```bash
brew tap d-pagey/mentees
brew install mentees
```

## 🧠 Why?

This project began as a learning tool to explore building full-featured CLIs in Rust. It helped me dive deep into:
- Command parsing with `clap`
- Working with `rusqlite` and local databases
- Clean architecture (services, models, repositories)
- CI/CD with GitHub Actions and Homebrew packaging

While it’s still a work-in-progress, it’s functional, fast, and fun to use!

## 📮 Feedback

Feel free to open issues or submit ideas — this was built for learning, but I’m happy to improve it for others too.
