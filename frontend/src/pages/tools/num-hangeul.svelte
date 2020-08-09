<script lang="ts">
  import { onMount } from 'svelte';
  import { fade } from 'svelte/transition';

  import wasm from '../../../korean_nums_wasm/Cargo.toml';
  import Button from "smelte/src/components/Button";
  import TextField from "smelte/src/components/TextField";
  import RadioButton from "smelte/src/components/RadioButton";
  import { Snackbar, notifier, Notifications } from "smelte";

  let exports;

  let num = '1';
  let hangeul = '일';
  let lower_bound = 1;
  let upper_bound = 10;

  let number_system_selected = 'sino';
  let mode_selected = 'number';
  let number_format_selected = 3;

  let showSnackbar = false;
  let answer_visible = false;
  let options_visible = false;

  let us_formatter = new Intl.NumberFormat(undefined, {
    minimumSignificantDigits: 4,
  })

  const number_systems = [
    { value: 'pure', label: 'Pure Korean' },
    { value: 'sino', label: 'Sino-Korean' },
  ]
  const modes = [
    { value: 'number', label: 'Number' },
    { value: 'money', label: '(Korean) Money' },
  ]
  const number_formats = [
    { value: 3, label: '3 Zeroes' },
    { value: 4, label: '4 Zeroes' },
  ]

  function notify(msg: String) {
    notifier.notify(msg)
  }

  function checkBounds(): Boolean {
    if (number_system_selected === 'pure') {
      if (lower_bound < 1) {
        notify('Cannot go lower than 1.');
        return false;
      }
      if (upper_bound > 99) {
        notify('Cannot go higher than 99 with Pure Korean numbers.')
        return false;
      }
    } else if (number_system_selected === 'sino') {
      if (upper_bound > Math.pow(2, 128)) {
        notify('Cannot go higher than 2^128.')
        return false;
      }
    }

    return true;
  }

  function getRandHangeul(): {} {
    if (checkBounds() === false) {
      return;
    }

    let res = JSON.parse(exports.random_korean_int(
            lower_bound.toString(), upper_bound.toString(), number_system_selected));
    num = res.number;
    hangeul = res.hangeul;
  }

  // 10000, 3 = 10,000
  // 10000, 4 = 1,0000
  function format_number(num: Number, place: Number): String {
    if (place === 3) {
      return String(num).replace(/(.)(?=(\d{3})+$)/g,'$1,');
    } else if (place === 4) {
      return String(num).replace(/(.)(?=(\d{4})+$)/g,'$1,');
    }
  }

  function handleClickRand() {
    answer_visible = false;
    getRandHangeul();
  }

  function handleClickToggleAnswer() {
    answer_visible = !answer_visible;
  }

  function handleClickToggleOptions() {
    options_visible = !options_visible;
  }

  onMount(async () => {
    exports = await wasm();
    exports.init_hook();
  })

</script>

<style>
  #num-to-hangul {
    margin: 0 auto;
  }
</style>

<Snackbar bind:value={showSnackbar}>
  <div>Have a nice day.</div>
  <div slot="action">
    <Button text on:click={() => (showSnackbar = false)}>Do something</Button>
  </div>
</Snackbar>

<br>
<div id="num-to-hangul" class="bg-white shadow-md rounded pb-8 text-center max-w-2xl">
  <h3> Korean Number Practice </h3>
  <div class="flex items-center">
    <div class="w-full md:w-1/2 px-3">
      <TextField
              outlined
              type="number"
              label="Lower Bound"
              bind:value={lower_bound}/>
    </div>
    <div class="w-full md:w-1/2 px-3">
      <TextField
              outlined
              type="number"
              max={Math.pow(2,128)}
              label="Upper Bound"
              bind:value={upper_bound}/>
    </div>
  </div>

  <div class="grid grid-rows-2">
    <h2 class="w-full">
      { format_number(num, number_format_selected) }
    </h2>
    {#if answer_visible}
      <h2 class="w-full" transition:fade id="answer">{hangeul}</h2>
    {/if}
  </div>

  <div >
    <Button color="primary" on:click={handleClickRand}> Random Number </Button>
    <Button color="primary" on:click={handleClickToggleAnswer}> Toggle Answer </Button>
    <Button color="secondary" on:click={handleClickToggleOptions}> Toggle Options </Button>
  </div>

  {#if options_visible}
    <div transition:fade class="flex items-center justify-center content-around">
      <div class="w-full md:w-1/3 p-2">
        <RadioButton
                color="secondary"
                name="Number System"
                bind:selected={number_system_selected}
                items={number_systems}/>
      </div>
      <div class="w-full md:w-1/3 p-2">
        <RadioButton
                color="secondary"
                name="Number Format"
                bind:selected={number_format_selected}
                items={number_formats}/>
      </div>
      <!-- <RadioButton
              name="Mode"
              bind:selected={mode_selected}
              items={modes}/>
              -->
    </div>
  {/if}

  <Notifications />
</div>
