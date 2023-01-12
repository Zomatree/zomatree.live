<script lang="ts">
    export let href: string;
    export let alt: string;
    export let image: string;
    export let title: string;
</script>

<style lang="scss">
    .blog-card {
        max-width: 400px;
        display: flex;
        flex-direction: column;
        align-items: center;
        background-color: var(--bg-2);
        border-radius: 5px;
        text-decoration: none;
        box-shadow: 2px 2px 8px 1px black;
    }

    .blog-card-image {
        border-top-left-radius: 5px;
        border-top-right-radius: 5px;
        max-width: 100%;
    }

    .blog-card-title {
        color: var(--primary-3)
    }
</style>

<a class="blog-card" href={href}>
    <img class="blog-card-image" alt={alt} src={image}/>
    <h3 class="blog-card-title">{title}</h3>
</a>
