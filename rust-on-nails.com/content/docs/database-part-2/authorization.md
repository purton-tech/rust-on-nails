+++
title = "Authorization (Row Level Security)"
description = "Authorization (Row Level Security)"
date = 2021-05-01T08:00:00+00:00
updated = 2021-05-01T08:00:00+00:00
draft = false
weight = 10
sort_by = "weight"

[extra]
toc = true
top = false
+++

## What problems are we trying to fix

User have roles. Some are administrators, others may only have ready only access to an application. Sometimes we need to split data between different users. i.e. In a CRM system I shouldn't be able to see contacts from other companies.

Let's define some terms for the different types of Authorization an application typically requires.

* Multi Tenancy. You could deploy a Postgres Instance for each user of an application. It would give good separation between each users data. However it's generally expected this this would add complexity and it's easier to share a database between all users. So multi tenancy is about how do we split data between users in a secure way.

* Data filtering. When a user is logged in to the system what data is the user allowed to see. Perhaps they only have permission to see the data they created and not their colleagues. Perhaps they can see data but not update it.

* User Interface. When a user is logged in, and they don't have permissions to perform certain actions we want to show that in the user interface so that we give them the best user experience.

## A quick look at Authorization libraries

There are a number of libraries and Start ups that are trying to fix the Authorization problem. Two popular examples are listed here.

1. Casbin https://casbin.org/
2. Oso https://www.osohq.com/

An initial assessment of these solutions is that they give you the answer to questions such as 

> Can this user perform this operation on this data.

What they don't have an answer for is

> What data can my user see?

This means to populate a web screen with a list of objects a user has permission to see, we would have to retrieve all the objects and pass them to the Authorization library.

Oso to be fair, has tried to address this issue by plugging into various frameworks more of which you can read about here https://www.osohq.com/post/authorization-logic-into-sql


## How do we tackle these issues with Postgres?

We already have Postgres let's lean in heavily and see what it can do.

### Multi Tenancy

You probably do this already. If you have an `organisations` table and a bunch of tables attached to it via an `organisation_id`. Then your queries to populate a web interface look something like

```sql
SELECT * FROM customers WHERE organisation_id = ?
```

That's multi tenancy. With Row Level Security we can add this restriction in one place and then all queries to the customer table going forward will have the restriction automatically applied.
We need to pass the current user_id to Postgres using something like

```sql
SET LOCAL row_level_security.user_id = 1234
```

As part of a transaction and then we can setup the Postgres policy.

```sql
CREATE POLICY multi_tenancy_policy_select ON customers FOR SELECT TO application
USING (
    organisation_id IN (SELECT get_orgs_for_app_user())
);
```

And then in our app we can shorten our query to

```sql
SELECT * FROM customers
```

### Data Filtering

After we have filtered our data by organisation we can further filter it based on arbitrary conditions such as a users role.

To achieve this we can model RBAC (Role Based Access Control) in the database and use those tables as part of our policies. The schema below implements a very simple RBAC in Postgres.

```sql
CREATE TYPE role AS ENUM (
    'Administrator', 
    'Collaborator', 
    'SystemAdministrator'
);
COMMENT ON TYPE role IS 'Users have roles, they can be managers or administrators etc.';

CREATE TYPE permission AS ENUM (
    -- The ManageTeam permission gives the user thee ability to invite team members, 
    -- delete team members and chnage the team name
    'ManageTeam'
);
COMMENT ON TYPE permission IS 'A permission gives the user the ability to do something. i.e. Manage users.';

CREATE TABLE roles_permissions (
    role role NOT NULL,
    permission permission NOT NULL,

    PRIMARY KEY (role, permission)
);
COMMENT ON TABLE roles_permissions IS 'Maps roles to permissions. i.e. a role can have multiple permissions.';
```

Now we have our roles in the database we can append them onto the policies.

So for example a policy that only gives select access to customers if the user is a SystemAdministrator

```sql
CREATE POLICY rbac_select ON customers FOR SELECT TO application
USING (
    'SystemAdministrator' IN SELECT roles FROM users WHERE user_id = current_setting(
            'row_level_security.user_id',
            false
        )::integer 

);
```

And we can repeat this pattern to cover all our Authorization needs.

### User Interface

As we now have the RBAC data in Postgres, we can use SQL to query whether a user has permission to do certain actions. Then  we pass the result to the user interface where we can disable menu items or remove buttons etc.

Something like the below query will give you all the permissions for a user.

```sql
SELECT permissions FROM roles_permissions WHERE role IN (SELECT roles FROM users WHERE user_id = ?)
```

## Conclusion

By putting our Policies and RBAC data next to our database tables we gain the ability to implement Authorization in a very concise way. We also get to re-use some of our existing knowledge i.e. SQL and Postgres.