<script lang="ts">
    import Highlight from "svelte-highlight";
    import type { Language } from "svelte-highlight/Highlight.svelte";
    import atom_one_dark from "svelte-highlight/styles/atom-one-dark";

    export let language: Language;
    export let code: string;
</script>

<svelte:head>
  {@html atom_one_dark}
</svelte:head>

<style lang="scss">
    .codeblock {
        margin-top: 20px;
        margin-bottom: 20px;
        padding: 5px;
        background-color: #282c34;
        border-radius: 5px;
        font-family: "Fira Mono";
        font-size: 14px;
    }

    :global(.codeblock > pre) {
        margin: 0;
    }

    :global(.hljs) {
        font-family: "Fira Mono";
    }
</style>

<div class="codeblock">
    <Highlight language={language} code={code}/>
</div>
