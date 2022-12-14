current_year := `date +%Y`
current_day := `date +%d`

# Run C solution of given year and day for the supplied input filename
run_c year day input_filename:
    clang "{{justfile_directory()}}/{{year}}/day{{day}}/solve.c" -o "{{justfile_directory()}}/{{year}}/day{{day}}/_solve"
    {{justfile_directory()}}/{{year}}/day{{day}}/_solve "{{justfile_directory()}}/{{year}}/day{{day}}/{{input_filename}}"

# Run rust solution of given year and day for the supplied input filename
run_rust year day input_filename:
    #!/usr/bin/env sh
    cd "{{justfile_directory()}}/{{year}}/day{{day}}"
    cargo run "{{justfile_directory()}}/{{year}}/day{{day}}/{{input_filename}}"

run_rust_single_file year day input_filename:
    rustc "{{justfile_directory()}}/{{year}}/day{{day}}/solve.rs" -o "{{justfile_directory()}}/{{year}}/day{{day}}/_solve"
    {{justfile_directory()}}/{{year}}/day{{day}}/_solve "{{justfile_directory()}}/{{year}}/day{{day}}/{{input_filename}}"

# Run python solution of given year and day for the supplied input filename
run_python year day input_filename:
    python3 "{{justfile_directory()}}/{{year}}/day{{day}}/solve.py" "{{justfile_directory()}}/{{year}}/day{{day}}/{{input_filename}}"

# Run ruby solution of given year and day for the supplied input filename
run_ruby year day input_filename:
    echo "Ruby is disabled"
    exit 1

# Run solution for supplied input filename in current directory or supplied year and day
run input_filename year="" day="" language="":
    #!/usr/bin/env sh
    d="{{trim_start_match(invocation_directory(), justfile_directory())}}"
    # Test if we're in the directory for a day
    if [ -n "$d" ] && [ $(expr $d : '^/[[:digit:]]\+/day[[:digit:]]\+') -gt 0 ]; then
        day="{{file_name(invocation_directory())}}"
        day="${day#day}" # Removes the "day" prefix from the variable named $day
        year="{{file_name(parent_directory(invocation_directory()))}}"
    # We can't figure out year and day ourselves
    elif [ -n "{{year}}" ] && [ -n "{{day}}" ]; then
        day="{{day}}"
        year="{{year}}"
    else
        echo "You must run this command from within a day's directory or provide the year and day as arguments."
        exit 1
    fi
    dir="{{justfile_directory()}}/$year/day$day"
    lang=""
    if [ -f "$dir/solve.rs" ]; then
        lang="rust_single_file"
    elif [ -f "$dir/Cargo.toml" ]; then
        lang="rust"
    elif [ -f "$dir/solve.c" ]; then
        lang="c"
    elif [ -f "$dir/solve.py" ]; then
        lang="python"
    elif [ -f "$dir/solve.rb" ]; then
        lang="ruby"
    fi
    if [ -z "$lang" ]; then
        echo "Couldn't figure out language automatically."
        exit 1
    fi
    just run_$lang $year $day "{{input_filename}}"

# Run solution on example file
test example_number="1" year="" day="" language="":
    #!/usr/bin/env sh
    cd "{{invocation_directory()}}"
    just run "example_{{example_number}}" {{year}} {{day}} "{{language}}"

# Run solution on input file
answer year="" day="" input_filename="input" language="":
    #!/usr/bin/env sh
    cd "{{invocation_directory()}}"
    just run "{{input_filename}}" {{year}} {{day}} "{{language}}"

# Create new directory for supplied day and year
new_dir day year:
    # mkdir -p "{{justfile_directory()}}/{{year}}/day{{day}}/"
    mkdir -p "{{justfile_directory()}}/{{year}}/"

# Copy rust template to directory for given day and year
new_rust day year:
    cargo new "{{justfile_directory()}}/{{year}}/day{{day}}"
    cp "{{justfile_directory()}}/templates/rust/solve.rs" "{{justfile_directory()}}/{{year}}/day{{day}}/src/main.rs"
    cp "{{justfile_directory()}}/templates/rust/.gitignore" "{{justfile_directory()}}/{{year}}/day{{day}}/.gitignore"

# Copy C template to directory for given day and year
new_c day year:
    cp -r "{{justfile_directory()}}/templates/c" "{{justfile_directory()}}/{{year}}/day{{day}}"

# Copy python template to directory for given day and year
new_python day year:
    cp -r "{{justfile_directory()}}/templates/python" "{{justfile_directory()}}/{{year}}/day{{day}}"

# Create new solution for today or supplied day and year in given language
new language day="" year="":
    #!/usr/bin/env sh
    # If no day was supplied & it's currently december, assume we want to create a new solution for today
    if [ -z "{{day}}" ] && [ $(date +%m) == 12 ]; then
        day="{{current_day}}"
    elif [ -n "{{day}}" ]; then
        day=$(printf "%02d" "{{day}}")
    else
        echo "It's not december and you didn't supply a day, idk what to do."
        exit 1
    fi
    # If no year was supplied, assume we want to create a new solution for the current year
    if [ -z "{{year}}" ]; then
        year="{{current_year}}"
    else
        year="{{year}}"
    fi
    # Test if we're in the directory for a year, if so, assume we want to create the solution for this year instead
    d="{{trim_start_match(invocation_directory(), justfile_directory())}}"
    if [ -n "$d" ] && [ $(expr $d : '^/[[:digit:]]\+/*$') -gt 0 ]; then
        year="{{file_name(invocation_directory())}}"
    fi
    just new_dir $day $year
    just new_{{language}} $day $year

# Get input
get year="" day="":
    #!/usr/bin/env sh
    if [ -f "{{justfile_directory()}}/session" ]; then
        session=$(cat "{{justfile_directory()}}/session")
    else
        echo "Session file not found, open network tab of browser dev tools, refresh page, click input, click cookies, copy value for session and paste in {{justfile_directory()}}/session"
        exit 1
    fi
    d="{{trim_start_match(invocation_directory(), justfile_directory())}}"
    # Test if we're in the directory for a day
    if [ -n "$d" ] && [ $(expr $d : '^/[[:digit:]]\+/day[[:digit:]]\+') -gt 0 ]; then
        day="{{file_name(invocation_directory())}}"
        day="${day#day}" # Removes the "day" prefix from the variable named $day
        year="{{file_name(parent_directory(invocation_directory()))}}"
    # We can't figure out year and day ourselves
    elif [ -n "{{year}}" ] && [ -n "{{day}}" ]; then
        day="{{day}}"
        year="{{year}}"
    else
        echo "You must run this command from within a day's directory or provide the year and day as arguments."
        exit 1
    fi
    dir="{{justfile_directory()}}/$year/day$day"
    day=$(echo "$day" | sed 's/0*//')
    curl "https://adventofcode.com/$year/day/$day/input" --cookie "session=$session" -o "$dir/input"

