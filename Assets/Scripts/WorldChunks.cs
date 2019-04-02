using UnityEngine;
using System.Collections;
using System.Collections.Generic;
using System;

[Serializable]
public class ChunkLookup : Dictionary<BlockPos, Chunk> { }

/// <summary>
/// Maintains the chunks data for an entire world.
/// </summary>
[Serializable]
public class WorldChunks
{
    [SerializeField] private readonly ChunkLookup chunks = new ChunkLookup();

    public World World { get; private set; }

    public WorldChunks(World world)
    {
        World = world;
    }

    /// <summary>
    /// Returns the chunk that contains the given position or null if the chunk is not present.
    /// </summary>
    public Chunk Get(BlockPos pos)
    {
        BlockPos chunkPos = pos.ContainingChunkCoordinates();

        Chunk result = null;
        chunks.TryGetValue(chunkPos, out result);
        return result;
    }

    /// <summary>
    /// Insert or update the chunk containing the given position.
    /// </summary>
    public void Set(BlockPos pos, Chunk chunk)
    {
        pos = pos.ContainingChunkCoordinates();
        chunks[pos] = chunk;
    }

    /// <summary>
    /// Removes the chunk containing the given position.
    /// </summary>
    public void Remove(BlockPos pos)
    {
        chunks.Remove(pos.ContainingChunkCoordinates());
    }
}
