+++
title = "Database Backups"
description = "Database Backups"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 140
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = 'Ideally we want to take regular backups of our database and also make those backups offsite. We can achieve this by using a backup service. This is how we set it up.'
toc = true
top = false
+++

## Creating a Readonly Database User

Our offsite backup provider needs readonly access to our database.

```
CREATE ROLE readonly LOGIN ENCRYPTED PASSWORD '****************';
```

```
GRANT SELECT ON ALL TABLES IN SCHEMA public TO readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO readonly;
```

## BackupSheep

[BackupSheep.com](https://backupsheep.com) gives us 3 database backups for free and allows is to set backup frequency and even to use your own external storage.

## Restore from a backup

Here we will update your local database.

1. Download one of the backups from BlackSheep.
1. `sudo apt-get install zip`
1. unzip the backup download i.e. `unzip bs*`
1. Drop the database `dbmate drop`
1. Recreate the database `dbmate create`
1. `psql $DATABASE_URL -f FILENAME.sql`

![Backup Sheep](/backup-sheep.png)