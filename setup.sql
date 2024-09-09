CREATE TABLE users(id SERIAL PRIMARY KEY, name varchar(20) UNIQUE KEY not null);
CREATE TABLE teams(id SERIAL PRIMARY KEY, name varchar(30) UNIQUE KEY not null);
CREATE TABLE players(id SERIAL PRIMARY KEY, user_id int not null REFERENCES users(id));
CREATE TABLE players_to_teams(player_id int not null REFERENCES players(id), team_id int not null REFERENCES teams(id), PRIMARY KEY (player_id, team_id));

CREATE TABLE available_blocks(
   id SERIAL PRIMARY KEY,
   start_time time,
   end_time time,
   needs_waring boolean,
   repeats varchar(50), -- rrule
   player_id int not null REFERENCES players(id)
);
