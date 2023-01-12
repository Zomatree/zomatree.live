<script lang="ts">
    import "./project.scss";

    export let title: string;
    export let description: string;
    export let tag: string;
    export let tag_colour: string;
    export let url: string | null = null;
    export let github_url: string;
</script>

<div class="project-card">
    <div class="title-row">
        <h2 class="project-title">
            { title }
        </h2>

        <div class="project-type" style="background-color: {tag_colour}">
            {tag}
        </div>
    </div>

    <p class="project-description">
        {description}
    </p>

    <p class="project-description">
        <a class="website-link" href={github_url} target="_blank" rel="nooopener noreferrer">Github </a>
        {#if url}
        • <a class="website-link" href={url} target="_blank" rel="nooopener noreferrer">Website</a>
        {/if}
    </p>
</div>
