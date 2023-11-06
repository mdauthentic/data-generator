_/!\ This is still a work in progress, some feature might change as things unfold during the development._

# Data Generator
Utility to generate fake data based a defined schema.
The schema syntax is inspired by the [EdgeDB](https://www.edgedb.com/showcase/data-modeling) data model syntax. 


User will define the schema, a `.sdl` as follows;

```
model test_data {
    col1: int(1..100),
    col2: str {
        provider  := @name,
        default   := one_of('my_name', 'another_str')
    },
    col3: currency(1..200) {
        provider := @currency,
        default  := '$3.40'
    },
    col4: array<str> {
        providers := [ @name, @business ],
        default   := many_of('s1', 's2', 's3'),
        maxlength := 10
    },
    col5: {datetime|date},
    col6: uuid,
    configuration {
        lines       := 1_000,
        format      := csv
        description := "Lorem ipsum ipsum..."
    }
}
```


## Supported Types
The supported types are as follows;
- `int` or `int(0..10)`: where `0..10` represents the required rannge that the generated number must not exceed
- `float`: floating point number
- `str`: a combination of alphanumeric characters
- `bool`: a `true` or `false`
- `uuid`: this represents a `uuid`
- `array<str|int|float>`: array of scalar types. It should be noted that arrays cannot contain other arrays
- `currency` or `currency(0..10)`: currency value. Also, when a boud e.g. (0..10) is specified, the generator only generates currency within this given range.
- `date|datetime`: date and datetime field

It should be noted that arrays cannot contain other arrays.