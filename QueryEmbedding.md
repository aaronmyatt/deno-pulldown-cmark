# Transcript Vectors

```json
{
    "openaiApiKey": "sk-",
    "openaiOrg": "org-"
}
```

How do we turn our transcript into big set of vectors that can be inserted into Postgres(Supabase) with pgvector.
- I hoped [Hugging Face's](https://huggingface.co/) Transformer.js tool would be a good open source option for this. The documentation seemed quite straight forward, but in the end it didn't seem to be compatible with Deno. I'll be watching the space in hope of leveraging Deno's recent WebGPU support!
  - [Hugging Face](https://huggingface.co/Supabase/gte-small).
  - ```js skip
    import { pipeline } from '@xenova/transformers';
    const pipe = await pipeline('feature-extraction', 'Supabase/gte-small');
    ```

## setupEnv
One small catch with the OpenAI API key is that it needs to be present as an environment variables, I think we can use `Deno.env` to load it into there at the start of the Pipe.
```js skip
> Deno.env
{
  get: [Function: getEnv],
  toObject: [Function: toObject],
  set: [Function: setEnv],
  has: [Function: has],
  delete: [Function: deleteEnv]
}
```

```ts
Deno.env.set('OPENAI_API_KEY', opts.config.openaiApiKey)
```

## queryFrom
This pipe will eventually sit in a Supabase edge function, as a `Deno.serve` server process. To test, however, we will want to be able to simulate execution from the repl, so falling back to basic `input` data will be reasonable
```ts
if(input.request){
  const data = await input.json;
  input.query = data.query;
}
```

## embedding
When a user types a query, or chat response, into our site, we want to match that query against our database to see if it matches any snippets from a transcript. That will require:

1) generating an embedding via OpenAI for the text they typed in
2) matching that embedding against all transcript derived embeddings in the database using Sementic Search
3) returning a subset of most closely matching results
++LATER++
4) Add the most closely matching transcript chunk as additional context to an OpenAI Chat Completion prompt so that it can generate more relevant and content integrated answers.


```js example OpenAI Embeddings response
{
  object: "list",
  data: [
    {
      object: "embedding",
      index: 0,
      embedding: [
          0.021123258,     0.02944838,  -0.0027718733,        0.0462,
         -0.025583599,   -0.012006171,    0.011036808,    0.05656522,
         -0.028688096,     0.03094361,    0.024139056,  0.0102765225,
         -0.007786588,   -0.060974877,     0.03122238,  -0.006931267,
          0.011892128,     0.00887633,    0.022821229,   0.006148807,
          0.013710477,   -0.021908887,   -0.038394403,  -0.013862534,
        -0.0147495335,   -0.012196242,    0.028814811,   0.045110255,
         -0.014052605,    0.007767581,      0.0686791,   -0.03895195,
           0.03094361,   -0.023226714,    0.028738782,    0.03834372,
         -0.047847286,     0.03900263,   -0.036062863, -0.0058510285,
          0.016168732, -0.00097015564,   -0.010162479,   0.052510366,
          0.006595474,   0.0010358887,    -0.05504465,    0.04959594,
          0.016561547,    0.042652003,    -0.07268327,   0.015940648,
          0.072125725,    0.059606362,    -0.01658689,  -0.027040811,
        -0.0070959954,   -0.011030472,   -0.025444213,   0.033883378,
          0.061684474,    0.026457926,   0.0038521118,   0.013875205,
         -0.015636533,   -0.019754745, -0.00019709999,  0.0077739167,
          0.025583599,    0.021566758,     0.03791289,   0.033401866,
         -0.019742073,    0.017537246,    0.012284942,  -0.037253976,
           0.01578859,    -0.01927323,    0.036443006,  -0.007818267,
         -0.024329128,    0.026331212,  -0.0114106145,  0.0077105593,
         -0.023226714,     -0.0219469,    -0.03596149, -0.0048911683,
          0.004463508,  -0.0025406198,   -0.017043062,    -0.0245192,
         -0.028434668,   -0.010941772,    0.013178278,  -0.009446544,
           0.04191706,    0.043032143,   0.0036873834,  -0.013571091,
        ... 1436 more items
      ]
    }
  ],
  model: "text-embedding-3-small",
  usage: { prompt_tokens: 111, total_tokens: 111 }
}
```

```ts
import OpenAI from "https://deno.land/x/openai@v4.29.2/mod.ts";
const client = new OpenAI({org: opts.config.openaiOrg});

input.embedding = await client.embeddings.create({model: 'text-embedding-3-small', input: input.query})
```

## matchDb


## returnTopMatches