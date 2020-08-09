<script>
  import {url} from '@sveltech/routify';
  import Button from "smelte/src/components/Button";
</script>

<p> Home ! </p>

<Button> I'm a button </Button>

<a href={$url('/tools/num-hangeul')}> Go to Num->Hangeul </a>
