-- actions_labels table
drop table if exists actions_labels;

-- actions table
drop trigger if exists actions_update_timestamps;
drop table if exists actions;

-- labels
drop trigger if exists labels_update_timestamps;
drop table if exists labels;