---
date: 2026-02-25
published: true
title: Spec Driven Data Engineering
---


*An auditable collaboration layer between humans and agents built to accelerate data engineering and analytics delivery*

## The Problem

Working in data engineering today often feels like a never-ending loop. The PM updates a source-to-target mapping (STTM) document. They hand it over to the dev team. There's something wrong and it goes back to the PM. They fix it, retry, and discover a null scenario nobody thought they had to handle. Rinse and repeat.

It's an inefficient and slow way of working. It creates an inevitable gap between an excel file (usually awfully formatted, ugly, and hard to read) to what's actually there in the code. Even worse. Excel is the wrong tool: no validation (not even spellcheck!). No version control, and locked into a vendor controlled format that's not easy for a machine to read.

## The Core Idea

Advances in LLMs, agentic programming and tools like Claude Code are showing a better solutions. What if instead of having this slow, monotonous, and error prone process the STTMs were plain text specs in structured markdown that agents can read, validate, and implement the code. You have a spec that becomes an executable pipeline, and fits directly into the workflow of the most advanced software engineering teams.

## How this Could Work

The stack I'm envisioning has 4 components: 

1. **Markdown specs** with schema-aware UI (autocomplete, validation, naming enforcement)
2. **Git** for version control and audit trail
3. **Agentic coding** tools for implementation
4. **Policy repository** ("constitution") for org-wide standards, glossary, governance rules

All of this fits directly into the tooling and Software Delivery Life Cycle (SDLC). You can integrate this into CI/CD, and finally have a clear diff of what changed from a requirement perspective. The whole time you have clear audit trails of everything. This feels like a boring point but is crucial if an agentic programing workflow can be adopted in highly regulated enterprises. 

"I don't know Claude updated the pipeline" is not going to cut it.

You start by creating a markdown spec document that outlines the requirements for a new analytical data mart: columns, transformations, SLAs, filter criteria etc. This shouldn't be a  manual exercise either, there should be templates, autocomplete, and then ideally a type of "linting" to guide the creator.

``` MARKDOWN 
# Spec: gold.enrollment_monthly_summary

## Description
Monthly enrollment counts aggregated by member demographics and plan attributes.
Grain: one row per calendar_month × age_band × gender × lob × plan_type × region.

## Source
- silver.member_enrollment_scd2

## Columns
| column | type | description |
|---|---|---|
| calendar_month | date | First day of month (2024-01-01) |
| age_band | string | 0-17, 18-25, 26-44, 45-64, 65+ |
| gender | string | M, F, U |
| line_of_business | string | Commercial, Medicare, Medicaid |
| plan_type | string | HMO, PPO, EPO, POS |
| region | string | Source: member.service_region |
| bom_member_count | int | Members active on 1st of month |
| eom_member_count | int | Members active on last day of month |
| member_months | decimal | Sum of fractional enrollment months |
| net_change | int | eom_member_count - bom_member_count |

## Transformation Logic
- Join member_enrollment_scd2 to dim_date on effective_date/term_date overlap
- BOM: member has active coverage where effective_date <= first_of_month AND (term_date >= first_of_month OR term_date IS NULL)
- EOM: same logic against last_day_of_month
- Member months: days_covered_in_month / days_in_month per member, then sum
- Age band derived from date_of_birth relative to calendar_month

## Filters
- Exclude records where enrollment_status = 'voided'
- Exclude coverage_days < 1 within the month

## SLA
- Refresh: daily, 06:00 UTC
- Latency: must reflect prior-day enrollment changes

```

The agent then takes the spec and can autogenerate the code, tests, and pipeline. In a matter of minutes you can have a working prototype of the data mart that you can interact with, confirm things are looking good. This will dramatically decrease the iteration cycle time on what you want the end product to look like. 

The feedback loop becomes even more powerful because the agent can work with you during the implementation. If it comes across an edge case that the spec didn't outline how to handle, it can ask, propose an approach, and even automatically update the spec. The data pipeline and spec grow organically as the model interacts with the data through its development process.

It's no longer a back and forth process but a collaboration between you and the agent using the spec to translate between natural language and code.

## Policy Repository

A key foundation that makes the vision possible is a well thought through  "Policy Repository". This enables you to guide the agent towards the best way to approach relatively simple problems (naming conventions) to complex problems (access controls or PHI detections). You can codify the standards you'd want a data engineer to follow, and because it's an agent writing the code you know they'll be automatically enforced. 

Below is an example of how this could look. The agent can then use these to add context to its workflow, and refer to them when it has questions. Think of a `CLAUDE.MD` on steroids where the agent can load the relevant policy and guidelines based on tags and references within the spec. Policies like naming conventions will almost always be referenced but this can expand to more specific data domain focused policies

``` policy_repo
/policies
  /governance
    access-control.md
    phi-masking.md
    retention.md
  /naming
    columns.md
    tables.md
    schemas.md
  /testing
    data-quality-checks.md
    coverage-requirements.md
  /transformations
    scd-handling.md
    null-defaults.md
```

``` Markdown
# Column Naming Standards

## Rules
- snake_case, lowercase only
- Boolean columns: prefix with `is_` or `has_` (e.g., `is_active`, `has_dependent`)
- Date columns: suffix with `_date` (e.g., `effective_date`, `term_date`)
- Count columns: suffix with `_count` (e.g., `member_count`, `claim_count`)
- Identifiers: suffix with `_id` or `_key` (e.g., `member_id`, `plan_sk`)
- No abbreviations unless in approved glossary (see /governance/glossary.md)

## Prohibited
- camelCase, PascalCase
- Generic names: `flag`, `status`, `type` without a prefix (use `enrollment_status`, `plan_type`)
```

Lastly this is the core feature that enables what [Dan Shipper](https://every.to/@danshipper) and [Kieran Klaassen](https://every.to/@kieran_1355) have called [compound engineering](https://every.to/chain-of-thought/compound-engineering-how-every-codes-with-agents). As you work in this new process you continue to review where the model went wrong, and codify that back into the policy and `CLAUDE.MD` style documents to ensure similar mistakes won't be made in the future. Effectively you are creating a learning system or flywheel that improves with every implementation cycle. 
## What Changes

The role of the data engineer in this world shifts its focus from writing PySpark code towards defining the analytical products that are built, performance tuning, and architecture design. Agents and LLMs write the code and work through the "how" of the implementation. Governance, naming conventions, and other policies are enforced as a byproduct of creating the spec, not 3 months after the fact when someone notices. 

This also enables increased transparency and auditability of the transformations, lineage, and policies in an organization's data warehouse. It's no longer tribal knowledge trapped in the heads of a few people. The audit trail of what is done and why becomes a byproduct of the work, and no longer an annoying word document that no one wants to maintain.

## Why Now

LLMs have reached an inflection point where they are capable enough to make this vision a reality. Anthropic is showing just how far the technology has advanced. If models can [write compilers from scratch](https://www.anthropic.com/engineering/building-c-compiler), I'd bet they can generate a well specified dbt model. With clear instructions and guardrails they can complete tasks that would've taken human engineers days and weeks to complete. 

Large companies and highly regulated companies will need an auditable human-agent collaboration workflow. A spec driven workflow (even if the agent made updates) will give you that traceability to show exactly what was done, and why.

## What's Hard & Open Questions

Now this is by no means going to be easy and there are still some big open questions that need to be answered for how this could get done.

1. **Policy language design** - there's been some great work on this topic from Anthropic [Claude Constitution](https://www.anthropic.com/constitution#acknowledgements). As mentioned above I'd bet properly codifying the policy repository to steer the LLM toward a company's goal will be a crucial skill.
2. **The interface problem** - There's a real need for a true product for this workflow that makes it as easy as possible to build specs, iterate on them with the model, and seamlessly integrate into a Git focused SDLC.
3. **Agent reliability for complex transformations** - This I'd bet gets solved by the continued model improvements in software development, but I think this perspective comes down to your confidence in the pace of model improvements.

These tools are getting more powerful and capable every day, and the rate of progress has been mind blowing. I think there's a ton of work to be done to reorient how software development is done to best take advantage of these new tools. I can see a world where data and analytics no longer is constrained on data engineering effort, but rather deep understanding of the data and problems you are trying to solve.

This is an opportunity where there is a clear need for tool and/or product to exist to take advantage of a new era of data engineering we're entering. If you're thinking about this problem, I'd love to talk: .
