## Diary

### Usage

🧰 Backup tool helps to easily backup and restore folders and files

```bash
$ cp config.example.yaml ~/.config/diary/config.yaml
$ diary add "sleeping hours"
$ diary show "sleeping hours"
$ diary edit "sleeping hours" --date 2000-01-01
```

### Environment variables

- `DIARY_CONFIG_PATH` - path to config file
- `DIARY_DB_PATH` - path to sqlite database
