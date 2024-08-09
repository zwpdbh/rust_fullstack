# SeaORM define entity


## Relationship 

To define the relationship correctly between `Workload` and `StorageType`, you need to use the appropriate relationship types provided by SeaOrm: `has_one`, `has_many`, and `belongs_to`. Here's a quick guideline on when to use each:

- **`has_one`**: Use this when an entity has a single related entity. For example, a `User` has one `Profile`.
- **`has_many`**: Use this when an entity has multiple related entities. For example, a `User` has many `Posts`.
- **`belongs_to`**: Use this when an entity belongs to another entity. For example, a `Post` belongs to a `User`.

In your case, where `Workload` has one particular `StorageType`, the relationship from `Workload` to `StorageType` is `belongs_to`, and from `StorageType` to `Workload`, it is `has_many`.

