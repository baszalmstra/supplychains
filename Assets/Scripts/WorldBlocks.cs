using UnityEngine;
using System.Collections;

/// <summary>
/// A utility class to be able to query and modify individual blocks instead of chunks.
/// </summary>
public class WorldBlocks
{
    public World World { get; private set; }

    public WorldBlocks(World world)
    {
        World = world;
    }

    /// <summary>
    /// Returns the Block at the given position.
    /// </summary>
    public Block Get(BlockPos pos)
    {
        if(pos.y < 0)
            return Block.BedRock;

        Chunk chunk = World.Chunks.Get(pos);
        if(chunk != null)
            return chunk.Get(pos);
        
        return Block.Void;
    }

    public void Set(BlockPos pos, Block block)
    {
        Chunk chunk = World.Chunks.Get(pos);
        if(chunk != null)
            chunk.Set(pos, block);
    }
}
