#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// I don't know C so this might be really terrible

typedef struct {
  char *content;
  char **lines;
  size_t line_count;
} Input;

typedef struct {
  int part_1;
  int part_2;
} Result;

Input read_file(char *path);
void solve(Input input, Result *result);

Input read_file(char *path) {
  // Get file handle
  FILE *fp;
  fp = fopen(path, "r");
  if (fp == NULL) {
    exit(EXIT_FAILURE);
  }

  // Get size of file in bytes
  fseek(fp, 0L, SEEK_END);
  size_t size = ftell(fp);
  fseek(fp, 0, SEEK_SET);

  // Allocate content buffer
  char *content = malloc(size * sizeof(char));

  // Iterate over lines in file
  int i = 0; // Keeps track of character
  ssize_t read;
  char *line = NULL;
  size_t len = 0;
  size_t line_count = 0;
  while ((read = getline(&line, &len, fp)) != -1) {
    for (int j = 0; j < read; j++) {
      content[i] = line[j];
      i++;
    }
    line_count++;
  }

  // Create array with pointer to every line in the string
  char **lines = malloc(line_count * sizeof(void *));

  // 'Split' string into lines
  // `strtok` will search for a delimiter, in this case \n, then return the
  // string up to that point. If you call it multiple times, it will operate
  // like a split function. While doing this, it will replace each occurence of
  // \n in the string with \0, essentially turning the memory into multiple
  // strings. We can take advantage of that by storing the address of the
  // beginning of each line in our **lines variable. Tbh I have no idea if this
  // is how you do it but I'm kind of proud I figured this out by myself.
  i = 0;
  char *token = strtok(content, "\n");
  while (token) {
    lines[i] = token;
    token = strtok(NULL, "\n");
    i++;
  }

  // Close file handle and free memory
  fclose(fp);
  if (line) {
    free(line);
  }

  Input result;
  result.content = content;
  result.lines = lines;
  result.line_count = line_count;
  return result;
}

int main(int argc, char *argv[]) {
  // Read input file
  Input input = read_file(argv[1]);
  // Declare result variable
  Result result;
  // Call the solve function which will assign values to the result var
  solve(input, &result);
  // Output results
  printf("Part 1: %d\n", result.part_1);
  printf("Part 2: %d\n", result.part_2);
  // Cleanup
  free(input.lines);
  free(input.content);
  // Exit
  exit(EXIT_SUCCESS);
}

typedef struct {
  int three_vowels;
  int twice_in_a_row;
  int does_not_contain;
  int pair_twice;
  int repeats_seperated;
} Flags;

Flags check_string(char *s) {
  Flags result = {0};
  int vowel_count = 0;
  int contains_pair = 0;
  int pairs[26][26] = {0};
  char *disallowed[4] = {"ab", "cd", "pq", "xy"};
  int contains_disallowed = 0;
  int contains_separated_pair = 0;
  int contains_double_pair = 0;
  int i = 0;
  char c;
  while ((c = s[i]) != '\0') {
    if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
      vowel_count += 1;
    }
    if (s[i + 1] == '\0') {
      ++i;
      continue;
    }
    if (c == s[i + 1]) {
      contains_pair = 1;
    }
    if (pairs[(int)c - 'a'][(int)s[i + 1] - 'a'] > 0) {
      if (pairs[(int)c - 'a'][(int)s[i + 1] - 'a'] != i) {
        contains_double_pair = 1;
      }
    } else {
      pairs[(int)c - 'a'][(int)s[i + 1] - 'a'] = i+1;
    }
    for (int j = 0; j < 4; j++) {
      if (c == disallowed[j][0] && s[i + 1] == disallowed[j][1]) {
        contains_disallowed = 1;
      }
    }
    if (s[i + 2] != '\0') {
      if (c == s[i + 2]) {
        contains_separated_pair = 1;
      }
    }
    ++i;
  }
  if (vowel_count >= 3) {
    result.three_vowels = 1;
  }
  if (contains_pair) {
    result.twice_in_a_row = 1;
  }
  if (!contains_disallowed) {
    result.does_not_contain = 1;
  }
  if (contains_double_pair) {
    result.pair_twice = 1;
  }
  if (contains_separated_pair) {
    result.repeats_seperated = 1;
  }
  return result;
}

void solve(Input input, Result *result) {
  for (int i = 0; i < input.line_count; i++) {
    Flags f = check_string(input.lines[i]);
    if (f.three_vowels && f.twice_in_a_row && f.does_not_contain) {
      result->part_1 += 1;
    }
    if (f.pair_twice && f.repeats_seperated) {
      result->part_2 += 1;
    }
  }
}
