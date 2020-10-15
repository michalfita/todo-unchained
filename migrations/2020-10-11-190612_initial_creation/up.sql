-- actions table
create table if not exists actions (
    id          character(36) not null primary key,
    title       varchar(256) not null,
    description text,
    
    created_at  datetime default current_timestamp not null,
    updated_at  datetime default current_timestamp not null,
    done_at     datetime
) without rowid;

create trigger if not exists actions_update_timestamps after update on actions
    for each row when new.updated_at <= old.updated_at
begin
    update actions set updated_at = current_timestamp where id = old.id;
end;

-- labels table
create table if not exists labels (
    id     character(36) not null primary key,
    parent character(36) references labels(id) on delete cascade,
    name   varchar(256) not null,
    created_at  datetime default current_timestamp not null,
    updated_at  datetime default current_timestamp not null
) without rowid;

create trigger if not exists labels_update_timestamps after update on labels
    for each row when new.updated_at <= old.updated_at
begin
    update labels set updated_at = current_timestamp where id = old.id;
end;

-- actions_labels table
create table if not exists actions_labels (
    id          character(36) not null primary key,
    action      character(36) not null references actions(id) on delete cascade,
    label       character(36) not null references labels(id) on delete cascade,
    created_at  datetime default current_timestamp not null,
    unique      (action, label)
) without rowid;
