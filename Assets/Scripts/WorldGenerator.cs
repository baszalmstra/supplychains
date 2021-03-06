﻿using System.Collections;
using System.Collections.Generic;
using UnityEngine;

/// <summary>
/// A class that is used to generate new chunks
/// </summary>
public class WorldGenerator
{
    public Chunk Generate(World world, BlockPos chunkPos)
    {
        Debug.Assert(chunkPos == chunkPos.ContainingChunkCoordinates());

        Chunk chunk = new Chunk(world, chunkPos);

        // First generate simple height based perlin noise
        for(int z = 0; z < Constants.ChunkSize; ++z)
            for(int x = 0; x < Constants.ChunkSize; ++x)
            {
                int globalX = x + chunkPos.x;
                int globalZ = z + chunkPos.z;

                float mountains = Mathf.Pow(Mathf.PerlinNoise((globalX+2238746)*0.02f, (globalZ+6879346)*0.02f), 1.5f);
                float steepness = Mathf.SmoothStep(0.0f, 1.0f, Mathf.PerlinNoise((globalX+78952)*0.002f, (globalZ+2957112)*0.002f));

                int height = (int)Mathf.Max(1, Mathf.Round(mountains * steepness * Constants.ChunkLayers));
                
                // Add blocks until the height was reached
                for(int y = 0; y < height; ++y)
                    chunk.Set(new BlockPos(globalX, y, globalZ), Block.Grass);
            }

        return chunk;
    }
}
